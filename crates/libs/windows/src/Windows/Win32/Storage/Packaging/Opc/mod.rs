#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOpcCertificateEnumerator(::windows_core::IUnknown);
impl IOpcCertificateEnumerator {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MoveNext)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MovePrevious(&self) -> ::windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MovePrevious)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub unsafe fn GetCurrent(&self) -> ::windows_core::Result<*mut super::super::super::Security::Cryptography::CERT_CONTEXT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCurrent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IOpcCertificateEnumerator> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IOpcCertificateEnumerator, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOpcCertificateEnumerator {
    type Vtable = IOpcCertificateEnumerator_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOpcCertificateEnumerator {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x85131937_8f24_421f_b439_59ab24d140b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcCertificateEnumerator_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MovePrevious: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MovePrevious: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub GetCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, certificate: *mut *mut super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography")))]
    GetCurrent: usize,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, copy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOpcCertificateSet(::windows_core::IUnknown);
impl IOpcCertificateSet {
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub unsafe fn Add(&self, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), certificate).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub unsafe fn Remove(&self, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), certificate).ok()
    }
    pub unsafe fn GetEnumerator(&self) -> ::windows_core::Result<IOpcCertificateEnumerator> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetEnumerator)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IOpcCertificateSet, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOpcCertificateSet {
    type Vtable = IOpcCertificateSet_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOpcCertificateSet {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x56ea4325_8e2d_4167_b1a4_e486d24c8fa7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcCertificateSet_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography")))]
    Add: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography")))]
    Remove: usize,
    pub GetEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, certificateenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOpcDigitalSignature(::windows_core::IUnknown);
impl IOpcDigitalSignature {
    pub unsafe fn GetNamespaces(&self, prefixes: *mut *mut ::windows_core::PWSTR, namespaces: *mut *mut ::windows_core::PWSTR, count: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetNamespaces)(::windows_core::Interface::as_raw(self), prefixes, namespaces, count).ok()
    }
    pub unsafe fn GetSignatureId(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSignatureId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSignaturePartName(&self) -> ::windows_core::Result<IOpcPartUri> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSignaturePartName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSignatureMethod(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSignatureMethod)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCanonicalizationMethod(&self) -> ::windows_core::Result<OPC_CANONICALIZATION_METHOD> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCanonicalizationMethod)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSignatureValue(&self, signaturevalue: *mut *mut u8, count: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSignatureValue)(::windows_core::Interface::as_raw(self), signaturevalue, count).ok()
    }
    pub unsafe fn GetSignaturePartReferenceEnumerator(&self) -> ::windows_core::Result<IOpcSignaturePartReferenceEnumerator> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSignaturePartReferenceEnumerator)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSignatureRelationshipReferenceEnumerator(&self) -> ::windows_core::Result<IOpcSignatureRelationshipReferenceEnumerator> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSignatureRelationshipReferenceEnumerator)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSigningTime(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSigningTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTimeFormat(&self) -> ::windows_core::Result<OPC_SIGNATURE_TIME_FORMAT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTimeFormat)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPackageObjectReference(&self) -> ::windows_core::Result<IOpcSignatureReference> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPackageObjectReference)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCertificateEnumerator(&self) -> ::windows_core::Result<IOpcCertificateEnumerator> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCertificateEnumerator)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCustomReferenceEnumerator(&self) -> ::windows_core::Result<IOpcSignatureReferenceEnumerator> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCustomReferenceEnumerator)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCustomObjectEnumerator(&self) -> ::windows_core::Result<IOpcSignatureCustomObjectEnumerator> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCustomObjectEnumerator)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSignatureXml(&self, signaturexml: *mut *mut u8, count: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSignatureXml)(::windows_core::Interface::as_raw(self), signaturexml, count).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IOpcDigitalSignature, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOpcDigitalSignature {
    type Vtable = IOpcDigitalSignature_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOpcDigitalSignature {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x52ab21dd_1cd0_4949_bc80_0c1232d00cb4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcDigitalSignature_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetNamespaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prefixes: *mut *mut ::windows_core::PWSTR, namespaces: *mut *mut ::windows_core::PWSTR, count: *mut u32) -> ::windows_core::HRESULT,
    pub GetSignatureId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signatureid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSignaturePartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturepartname: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSignaturePartName: usize,
    pub GetSignatureMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturemethod: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetCanonicalizationMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, canonicalizationmethod: *mut OPC_CANONICALIZATION_METHOD) -> ::windows_core::HRESULT,
    pub GetSignatureValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturevalue: *mut *mut u8, count: *mut u32) -> ::windows_core::HRESULT,
    pub GetSignaturePartReferenceEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partreferenceenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetSignatureRelationshipReferenceEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relationshipreferenceenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetSigningTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signingtime: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetTimeFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeformat: *mut OPC_SIGNATURE_TIME_FORMAT) -> ::windows_core::HRESULT,
    pub GetPackageObjectReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packageobjectreference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetCertificateEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, certificateenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetCustomReferenceEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customreferenceenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetCustomObjectEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customobjectenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetSignatureXml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturexml: *mut *mut u8, count: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOpcDigitalSignatureEnumerator(::windows_core::IUnknown);
impl IOpcDigitalSignatureEnumerator {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MoveNext)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MovePrevious(&self) -> ::windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MovePrevious)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCurrent(&self) -> ::windows_core::Result<IOpcDigitalSignature> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCurrent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IOpcDigitalSignatureEnumerator> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IOpcDigitalSignatureEnumerator, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOpcDigitalSignatureEnumerator {
    type Vtable = IOpcDigitalSignatureEnumerator_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOpcDigitalSignatureEnumerator {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x967b6882_0ba3_4358_b9e7_b64c75063c5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcDigitalSignatureEnumerator_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MovePrevious: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MovePrevious: usize,
    pub GetCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, digitalsignature: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, copy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOpcDigitalSignatureManager(::windows_core::IUnknown);
impl IOpcDigitalSignatureManager {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSignatureOriginPartName(&self) -> ::windows_core::Result<IOpcPartUri> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSignatureOriginPartName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSignatureOriginPartName<P0>(&self, signatureoriginpartname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IOpcPartUri>,
    {
        (::windows_core::Interface::vtable(self).SetSignatureOriginPartName)(::windows_core::Interface::as_raw(self), signatureoriginpartname.into_param().abi()).ok()
    }
    pub unsafe fn GetSignatureEnumerator(&self) -> ::windows_core::Result<IOpcDigitalSignatureEnumerator> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSignatureEnumerator)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RemoveSignature<P0>(&self, signaturepartname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IOpcPartUri>,
    {
        (::windows_core::Interface::vtable(self).RemoveSignature)(::windows_core::Interface::as_raw(self), signaturepartname.into_param().abi()).ok()
    }
    pub unsafe fn CreateSigningOptions(&self) -> ::windows_core::Result<IOpcSigningOptions> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateSigningOptions)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub unsafe fn Validate<P0>(&self, signature: P0, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows_core::Result<OPC_SIGNATURE_VALIDATION_RESULT>
    where
        P0: ::windows_core::IntoParam<IOpcDigitalSignature>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Validate)(::windows_core::Interface::as_raw(self), signature.into_param().abi(), certificate, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub unsafe fn Sign<P0>(&self, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT, signingoptions: P0) -> ::windows_core::Result<IOpcDigitalSignature>
    where
        P0: ::windows_core::IntoParam<IOpcSigningOptions>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Sign)(::windows_core::Interface::as_raw(self), certificate, signingoptions.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ReplaceSignatureXml<P0>(&self, signaturepartname: P0, newsignaturexml: &[u8]) -> ::windows_core::Result<IOpcDigitalSignature>
    where
        P0: ::windows_core::IntoParam<IOpcPartUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ReplaceSignatureXml)(::windows_core::Interface::as_raw(self), signaturepartname.into_param().abi(), ::core::mem::transmute(newsignaturexml.as_ptr()), newsignaturexml.len().try_into().unwrap(), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IOpcDigitalSignatureManager, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOpcDigitalSignatureManager {
    type Vtable = IOpcDigitalSignatureManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOpcDigitalSignatureManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd5e62a0b_696d_462f_94df_72e33cef2659);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcDigitalSignatureManager_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSignatureOriginPartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signatureoriginpartname: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSignatureOriginPartName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSignatureOriginPartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signatureoriginpartname: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSignatureOriginPartName: usize,
    pub GetSignatureEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signatureenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RemoveSignature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturepartname: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RemoveSignature: usize,
    pub CreateSigningOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signingoptions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub Validate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signature: *mut ::core::ffi::c_void, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT, validationresult: *mut OPC_SIGNATURE_VALIDATION_RESULT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography")))]
    Validate: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub Sign: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT, signingoptions: *mut ::core::ffi::c_void, digitalsignature: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography")))]
    Sign: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ReplaceSignatureXml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturepartname: *mut ::core::ffi::c_void, newsignaturexml: *const u8, count: u32, digitalsignature: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ReplaceSignatureXml: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOpcFactory(::windows_core::IUnknown);
impl IOpcFactory {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePackageRootUri(&self) -> ::windows_core::Result<IOpcUri> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreatePackageRootUri)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePartUri<P0>(&self, pwzuri: P0) -> ::windows_core::Result<IOpcPartUri>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreatePartUri)(::windows_core::Interface::as_raw(self), pwzuri.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Com"))]
    pub unsafe fn CreateStreamOnFile<P0>(&self, filename: P0, iomode: OPC_STREAM_IO_MODE, securityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, dwflagsandattributes: u32) -> ::windows_core::Result<super::super::super::System::Com::IStream>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateStreamOnFile)(::windows_core::Interface::as_raw(self), filename.into_param().abi(), iomode, securityattributes, dwflagsandattributes, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreatePackage(&self) -> ::windows_core::Result<IOpcPackage> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreatePackage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ReadPackageFromStream<P0>(&self, stream: P0, flags: OPC_READ_FLAGS) -> ::windows_core::Result<IOpcPackage>
    where
        P0: ::windows_core::IntoParam<super::super::super::System::Com::IStream>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ReadPackageFromStream)(::windows_core::Interface::as_raw(self), stream.into_param().abi(), flags, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn WritePackageToStream<P0, P1>(&self, package: P0, flags: OPC_WRITE_FLAGS, stream: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IOpcPackage>,
        P1: ::windows_core::IntoParam<super::super::super::System::Com::IStream>,
    {
        (::windows_core::Interface::vtable(self).WritePackageToStream)(::windows_core::Interface::as_raw(self), package.into_param().abi(), flags, stream.into_param().abi()).ok()
    }
    pub unsafe fn CreateDigitalSignatureManager<P0>(&self, package: P0) -> ::windows_core::Result<IOpcDigitalSignatureManager>
    where
        P0: ::windows_core::IntoParam<IOpcPackage>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateDigitalSignatureManager)(::windows_core::Interface::as_raw(self), package.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IOpcFactory, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOpcFactory {
    type Vtable = IOpcFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOpcFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6d0b4446_cd73_4ab3_94f4_8ccdf6116154);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcFactory_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreatePackageRootUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rooturi: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreatePackageRootUri: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreatePartUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzuri: ::windows_core::PCWSTR, parturi: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreatePartUri: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Com"))]
    pub CreateStreamOnFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, iomode: OPC_STREAM_IO_MODE, securityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, dwflagsandattributes: u32, stream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Com")))]
    CreateStreamOnFile: usize,
    pub CreatePackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, package: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ReadPackageFromStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, flags: OPC_READ_FLAGS, package: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ReadPackageFromStream: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub WritePackageToStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, package: *mut ::core::ffi::c_void, flags: OPC_WRITE_FLAGS, stream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    WritePackageToStream: usize,
    pub CreateDigitalSignatureManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, package: *mut ::core::ffi::c_void, signaturemanager: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOpcPackage(::windows_core::IUnknown);
impl IOpcPackage {
    pub unsafe fn GetPartSet(&self) -> ::windows_core::Result<IOpcPartSet> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPartSet)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRelationshipSet(&self) -> ::windows_core::Result<IOpcRelationshipSet> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRelationshipSet)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IOpcPackage, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOpcPackage {
    type Vtable = IOpcPackage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOpcPackage {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x42195949_3b79_4fc8_89c6_fc7fb979ee70);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcPackage_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetPartSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetRelationshipSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relationshipset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOpcPart(::windows_core::IUnknown);
impl IOpcPart {
    pub unsafe fn GetRelationshipSet(&self) -> ::windows_core::Result<IOpcRelationshipSet> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRelationshipSet)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetContentStream(&self) -> ::windows_core::Result<super::super::super::System::Com::IStream> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetContentStream)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetName(&self) -> ::windows_core::Result<IOpcPartUri> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetContentType(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetContentType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCompressionOptions(&self) -> ::windows_core::Result<OPC_COMPRESSION_OPTIONS> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCompressionOptions)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IOpcPart, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOpcPart {
    type Vtable = IOpcPart_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOpcPart {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x42195949_3b79_4fc8_89c6_fc7fb979ee71);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcPart_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetRelationshipSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relationshipset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetContentStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetContentStream: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetName: usize,
    pub GetContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contenttype: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetCompressionOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compressionoptions: *mut OPC_COMPRESSION_OPTIONS) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOpcPartEnumerator(::windows_core::IUnknown);
impl IOpcPartEnumerator {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MoveNext)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MovePrevious(&self) -> ::windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MovePrevious)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCurrent(&self) -> ::windows_core::Result<IOpcPart> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCurrent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IOpcPartEnumerator> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IOpcPartEnumerator, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOpcPartEnumerator {
    type Vtable = IOpcPartEnumerator_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOpcPartEnumerator {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x42195949_3b79_4fc8_89c6_fc7fb979ee75);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcPartEnumerator_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MovePrevious: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MovePrevious: usize,
    pub GetCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, part: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, copy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOpcPartSet(::windows_core::IUnknown);
impl IOpcPartSet {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPart<P0>(&self, name: P0) -> ::windows_core::Result<IOpcPart>
    where
        P0: ::windows_core::IntoParam<IOpcPartUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPart)(::windows_core::Interface::as_raw(self), name.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePart<P0, P1>(&self, name: P0, contenttype: P1, compressionoptions: OPC_COMPRESSION_OPTIONS) -> ::windows_core::Result<IOpcPart>
    where
        P0: ::windows_core::IntoParam<IOpcPartUri>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreatePart)(::windows_core::Interface::as_raw(self), name.into_param().abi(), contenttype.into_param().abi(), compressionoptions, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DeletePart<P0>(&self, name: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IOpcPartUri>,
    {
        (::windows_core::Interface::vtable(self).DeletePart)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn PartExists<P0>(&self, name: P0) -> ::windows_core::Result<super::super::super::Foundation::BOOL>
    where
        P0: ::windows_core::IntoParam<IOpcPartUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PartExists)(::windows_core::Interface::as_raw(self), name.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetEnumerator(&self) -> ::windows_core::Result<IOpcPartEnumerator> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetEnumerator)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IOpcPartSet, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOpcPartSet {
    type Vtable = IOpcPartSet_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOpcPartSet {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x42195949_3b79_4fc8_89c6_fc7fb979ee73);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcPartSet_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetPart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, part: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetPart: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreatePart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, contenttype: ::windows_core::PCWSTR, compressionoptions: OPC_COMPRESSION_OPTIONS, part: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreatePart: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DeletePart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DeletePart: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub PartExists: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, partexists: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    PartExists: usize,
    pub GetEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOpcPartUri(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IOpcPartUri {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPropertyBSTR(&self, uriprop: super::super::super::System::Com::Uri_PROPERTY, pbstrproperty: *mut ::windows_core::BSTR, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetPropertyBSTR)(::windows_core::Interface::as_raw(self), uriprop, ::core::mem::transmute(pbstrproperty), dwflags).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPropertyLength(&self, uriprop: super::super::super::System::Com::Uri_PROPERTY, pcchproperty: *mut u32, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetPropertyLength)(::windows_core::Interface::as_raw(self), uriprop, pcchproperty, dwflags).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPropertyDWORD(&self, uriprop: super::super::super::System::Com::Uri_PROPERTY, pdwproperty: *mut u32, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetPropertyDWORD)(::windows_core::Interface::as_raw(self), uriprop, pdwproperty, dwflags).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn HasProperty(&self, uriprop: super::super::super::System::Com::Uri_PROPERTY) -> ::windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.HasProperty)(::windows_core::Interface::as_raw(self), uriprop, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetAbsoluteUri(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetAbsoluteUri)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetAuthority(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetAuthority)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDisplayUri(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetDisplayUri)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDomain(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetDomain)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetExtension(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetExtension)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFragment(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetFragment)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetHost(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetHost)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPassword(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetPassword)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPathAndQuery(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetPathAndQuery)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetQuery(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetQuery)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRawUri(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetRawUri)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSchemeName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetSchemeName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetUserInfo(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetUserInfo)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetUserName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetUserName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetHostType(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetHostType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPort(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetPort)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetScheme(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetScheme)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetZone(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetZone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetProperties(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetProperties)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn IsEqual<P0>(&self, puri: P0) -> ::windows_core::Result<super::super::super::Foundation::BOOL>
    where
        P0: ::windows_core::IntoParam<super::super::super::System::Com::IUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.IsEqual)(::windows_core::Interface::as_raw(self), puri.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRelationshipsPartUri(&self) -> ::windows_core::Result<IOpcPartUri> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetRelationshipsPartUri)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRelativeUri<P0>(&self, targetparturi: P0) -> ::windows_core::Result<super::super::super::System::Com::IUri>
    where
        P0: ::windows_core::IntoParam<IOpcPartUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetRelativeUri)(::windows_core::Interface::as_raw(self), targetparturi.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CombinePartUri<P0>(&self, relativeuri: P0) -> ::windows_core::Result<IOpcPartUri>
    where
        P0: ::windows_core::IntoParam<super::super::super::System::Com::IUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CombinePartUri)(::windows_core::Interface::as_raw(self), relativeuri.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ComparePartUri<P0>(&self, parturi: P0) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<IOpcPartUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ComparePartUri)(::windows_core::Interface::as_raw(self), parturi.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSourceUri(&self) -> ::windows_core::Result<IOpcUri> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSourceUri)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRelationshipsPartUri(&self) -> ::windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsRelationshipsPartUri)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IOpcPartUri, ::windows_core::IUnknown, super::super::super::System::Com::IUri, IOpcUri);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IOpcPartUri {
    type Vtable = IOpcPartUri_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IOpcPartUri {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7d3babe7_88b2_46ba_85cb_4203cb016c87);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IOpcPartUri_Vtbl {
    pub base__: IOpcUri_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ComparePartUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, comparisonresult: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ComparePartUri: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSourceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceuri: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSourceUri: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsRelationshipsPartUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isrelationshipuri: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsRelationshipsPartUri: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOpcRelationship(::windows_core::IUnknown);
impl IOpcRelationship {
    pub unsafe fn GetId(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRelationshipType(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRelationshipType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSourceUri(&self) -> ::windows_core::Result<IOpcUri> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSourceUri)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTargetUri(&self) -> ::windows_core::Result<super::super::super::System::Com::IUri> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTargetUri)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTargetMode(&self) -> ::windows_core::Result<OPC_URI_TARGET_MODE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTargetMode)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IOpcRelationship, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOpcRelationship {
    type Vtable = IOpcRelationship_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOpcRelationship {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x42195949_3b79_4fc8_89c6_fc7fb979ee72);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcRelationship_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relationshipidentifier: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetRelationshipType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relationshiptype: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSourceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceuri: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSourceUri: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetTargetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targeturi: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetTargetUri: usize,
    pub GetTargetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetmode: *mut OPC_URI_TARGET_MODE) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOpcRelationshipEnumerator(::windows_core::IUnknown);
impl IOpcRelationshipEnumerator {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MoveNext)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MovePrevious(&self) -> ::windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MovePrevious)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCurrent(&self) -> ::windows_core::Result<IOpcRelationship> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCurrent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IOpcRelationshipEnumerator> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IOpcRelationshipEnumerator, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOpcRelationshipEnumerator {
    type Vtable = IOpcRelationshipEnumerator_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOpcRelationshipEnumerator {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x42195949_3b79_4fc8_89c6_fc7fb979ee76);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcRelationshipEnumerator_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MovePrevious: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MovePrevious: usize,
    pub GetCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relationship: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, copy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOpcRelationshipSelector(::windows_core::IUnknown);
impl IOpcRelationshipSelector {
    pub unsafe fn GetSelectorType(&self) -> ::windows_core::Result<OPC_RELATIONSHIP_SELECTOR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSelectorType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSelectionCriterion(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSelectionCriterion)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IOpcRelationshipSelector, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOpcRelationshipSelector {
    type Vtable = IOpcRelationshipSelector_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOpcRelationshipSelector {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf8f26c7f_b28f_4899_84c8_5d5639ede75f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcRelationshipSelector_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetSelectorType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selector: *mut OPC_RELATIONSHIP_SELECTOR) -> ::windows_core::HRESULT,
    pub GetSelectionCriterion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selectioncriterion: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOpcRelationshipSelectorEnumerator(::windows_core::IUnknown);
impl IOpcRelationshipSelectorEnumerator {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MoveNext)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MovePrevious(&self) -> ::windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MovePrevious)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCurrent(&self) -> ::windows_core::Result<IOpcRelationshipSelector> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCurrent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IOpcRelationshipSelectorEnumerator> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IOpcRelationshipSelectorEnumerator, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOpcRelationshipSelectorEnumerator {
    type Vtable = IOpcRelationshipSelectorEnumerator_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOpcRelationshipSelectorEnumerator {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5e50a181_a91b_48ac_88d2_bca3d8f8c0b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcRelationshipSelectorEnumerator_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MovePrevious: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MovePrevious: usize,
    pub GetCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relationshipselector: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, copy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOpcRelationshipSelectorSet(::windows_core::IUnknown);
impl IOpcRelationshipSelectorSet {
    pub unsafe fn Create<P0>(&self, selector: OPC_RELATIONSHIP_SELECTOR, selectioncriterion: P0) -> ::windows_core::Result<IOpcRelationshipSelector>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Create)(::windows_core::Interface::as_raw(self), selector, selectioncriterion.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Delete<P0>(&self, relationshipselector: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IOpcRelationshipSelector>,
    {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self), relationshipselector.into_param().abi()).ok()
    }
    pub unsafe fn GetEnumerator(&self) -> ::windows_core::Result<IOpcRelationshipSelectorEnumerator> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetEnumerator)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IOpcRelationshipSelectorSet, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOpcRelationshipSelectorSet {
    type Vtable = IOpcRelationshipSelectorSet_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOpcRelationshipSelectorSet {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6e34c269_a4d3_47c0_b5c4_87ff2b3b6136);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcRelationshipSelectorSet_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selector: OPC_RELATIONSHIP_SELECTOR, selectioncriterion: ::windows_core::PCWSTR, relationshipselector: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relationshipselector: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relationshipselectorenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOpcRelationshipSet(::windows_core::IUnknown);
impl IOpcRelationshipSet {
    pub unsafe fn GetRelationship<P0>(&self, relationshipidentifier: P0) -> ::windows_core::Result<IOpcRelationship>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRelationship)(::windows_core::Interface::as_raw(self), relationshipidentifier.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateRelationship<P0, P1, P2>(&self, relationshipidentifier: P0, relationshiptype: P1, targeturi: P2, targetmode: OPC_URI_TARGET_MODE) -> ::windows_core::Result<IOpcRelationship>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<super::super::super::System::Com::IUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateRelationship)(::windows_core::Interface::as_raw(self), relationshipidentifier.into_param().abi(), relationshiptype.into_param().abi(), targeturi.into_param().abi(), targetmode, &mut result__).from_abi(result__)
    }
    pub unsafe fn DeleteRelationship<P0>(&self, relationshipidentifier: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).DeleteRelationship)(::windows_core::Interface::as_raw(self), relationshipidentifier.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RelationshipExists<P0>(&self, relationshipidentifier: P0) -> ::windows_core::Result<super::super::super::Foundation::BOOL>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RelationshipExists)(::windows_core::Interface::as_raw(self), relationshipidentifier.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetEnumerator(&self) -> ::windows_core::Result<IOpcRelationshipEnumerator> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetEnumerator)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetEnumeratorForType<P0>(&self, relationshiptype: P0) -> ::windows_core::Result<IOpcRelationshipEnumerator>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetEnumeratorForType)(::windows_core::Interface::as_raw(self), relationshiptype.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRelationshipsContentStream(&self) -> ::windows_core::Result<super::super::super::System::Com::IStream> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRelationshipsContentStream)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IOpcRelationshipSet, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOpcRelationshipSet {
    type Vtable = IOpcRelationshipSet_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOpcRelationshipSet {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x42195949_3b79_4fc8_89c6_fc7fb979ee74);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcRelationshipSet_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetRelationship: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relationshipidentifier: ::windows_core::PCWSTR, relationship: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateRelationship: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relationshipidentifier: ::windows_core::PCWSTR, relationshiptype: ::windows_core::PCWSTR, targeturi: *mut ::core::ffi::c_void, targetmode: OPC_URI_TARGET_MODE, relationship: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateRelationship: usize,
    pub DeleteRelationship: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relationshipidentifier: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RelationshipExists: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relationshipidentifier: ::windows_core::PCWSTR, relationshipexists: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RelationshipExists: usize,
    pub GetEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relationshipenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetEnumeratorForType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relationshiptype: ::windows_core::PCWSTR, relationshipenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRelationshipsContentStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contents: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRelationshipsContentStream: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOpcSignatureCustomObject(::windows_core::IUnknown);
impl IOpcSignatureCustomObject {
    pub unsafe fn GetXml(&self, xmlmarkup: *mut *mut u8, count: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetXml)(::windows_core::Interface::as_raw(self), xmlmarkup, count).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IOpcSignatureCustomObject, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOpcSignatureCustomObject {
    type Vtable = IOpcSignatureCustomObject_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOpcSignatureCustomObject {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5d77a19e_62c1_44e7_becd_45da5ae51a56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignatureCustomObject_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetXml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xmlmarkup: *mut *mut u8, count: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOpcSignatureCustomObjectEnumerator(::windows_core::IUnknown);
impl IOpcSignatureCustomObjectEnumerator {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MoveNext)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MovePrevious(&self) -> ::windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MovePrevious)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCurrent(&self) -> ::windows_core::Result<IOpcSignatureCustomObject> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCurrent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IOpcSignatureCustomObjectEnumerator> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IOpcSignatureCustomObjectEnumerator, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOpcSignatureCustomObjectEnumerator {
    type Vtable = IOpcSignatureCustomObjectEnumerator_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOpcSignatureCustomObjectEnumerator {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5ee4fe1d_e1b0_4683_8079_7ea0fcf80b4c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignatureCustomObjectEnumerator_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MovePrevious: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MovePrevious: usize,
    pub GetCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, copy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOpcSignatureCustomObjectSet(::windows_core::IUnknown);
impl IOpcSignatureCustomObjectSet {
    pub unsafe fn Create(&self, xmlmarkup: &[u8]) -> ::windows_core::Result<IOpcSignatureCustomObject> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Create)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(xmlmarkup.as_ptr()), xmlmarkup.len().try_into().unwrap(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Delete<P0>(&self, customobject: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IOpcSignatureCustomObject>,
    {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self), customobject.into_param().abi()).ok()
    }
    pub unsafe fn GetEnumerator(&self) -> ::windows_core::Result<IOpcSignatureCustomObjectEnumerator> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetEnumerator)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IOpcSignatureCustomObjectSet, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOpcSignatureCustomObjectSet {
    type Vtable = IOpcSignatureCustomObjectSet_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOpcSignatureCustomObjectSet {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8f792ac5_7947_4e11_bc3d_2659ff046ae1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignatureCustomObjectSet_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xmlmarkup: *const u8, count: u32, customobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customobject: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customobjectenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOpcSignaturePartReference(::windows_core::IUnknown);
impl IOpcSignaturePartReference {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPartName(&self) -> ::windows_core::Result<IOpcPartUri> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPartName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetContentType(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetContentType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDigestMethod(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDigestMethod)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDigestValue(&self, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDigestValue)(::windows_core::Interface::as_raw(self), digestvalue, count).ok()
    }
    pub unsafe fn GetTransformMethod(&self) -> ::windows_core::Result<OPC_CANONICALIZATION_METHOD> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTransformMethod)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IOpcSignaturePartReference, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOpcSignaturePartReference {
    type Vtable = IOpcSignaturePartReference_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOpcSignaturePartReference {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe24231ca_59f4_484e_b64b_36eeda36072c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignaturePartReference_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetPartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partname: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetPartName: usize,
    pub GetContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contenttype: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetDigestMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, digestmethod: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetDigestValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows_core::HRESULT,
    pub GetTransformMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transformmethod: *mut OPC_CANONICALIZATION_METHOD) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOpcSignaturePartReferenceEnumerator(::windows_core::IUnknown);
impl IOpcSignaturePartReferenceEnumerator {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MoveNext)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MovePrevious(&self) -> ::windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MovePrevious)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCurrent(&self) -> ::windows_core::Result<IOpcSignaturePartReference> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCurrent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IOpcSignaturePartReferenceEnumerator> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IOpcSignaturePartReferenceEnumerator, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOpcSignaturePartReferenceEnumerator {
    type Vtable = IOpcSignaturePartReferenceEnumerator_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOpcSignaturePartReferenceEnumerator {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x80eb1561_8c77_49cf_8266_459b356ee99a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignaturePartReferenceEnumerator_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MovePrevious: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MovePrevious: usize,
    pub GetCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partreference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, copy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOpcSignaturePartReferenceSet(::windows_core::IUnknown);
impl IOpcSignaturePartReferenceSet {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Create<P0, P1>(&self, parturi: P0, digestmethod: P1, transformmethod: OPC_CANONICALIZATION_METHOD) -> ::windows_core::Result<IOpcSignaturePartReference>
    where
        P0: ::windows_core::IntoParam<IOpcPartUri>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Create)(::windows_core::Interface::as_raw(self), parturi.into_param().abi(), digestmethod.into_param().abi(), transformmethod, &mut result__).from_abi(result__)
    }
    pub unsafe fn Delete<P0>(&self, partreference: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IOpcSignaturePartReference>,
    {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self), partreference.into_param().abi()).ok()
    }
    pub unsafe fn GetEnumerator(&self) -> ::windows_core::Result<IOpcSignaturePartReferenceEnumerator> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetEnumerator)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IOpcSignaturePartReferenceSet, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOpcSignaturePartReferenceSet {
    type Vtable = IOpcSignaturePartReferenceSet_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOpcSignaturePartReferenceSet {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6c9fe28c_ecd9_4b22_9d36_7fdde670fec0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignaturePartReferenceSet_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, digestmethod: ::windows_core::PCWSTR, transformmethod: OPC_CANONICALIZATION_METHOD, partreference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Create: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partreference: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partreferenceenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOpcSignatureReference(::windows_core::IUnknown);
impl IOpcSignatureReference {
    pub unsafe fn GetId(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetUri(&self) -> ::windows_core::Result<super::super::super::System::Com::IUri> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetUri)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTransformMethod(&self) -> ::windows_core::Result<OPC_CANONICALIZATION_METHOD> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTransformMethod)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDigestMethod(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDigestMethod)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDigestValue(&self, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDigestValue)(::windows_core::Interface::as_raw(self), digestvalue, count).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IOpcSignatureReference, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOpcSignatureReference {
    type Vtable = IOpcSignatureReference_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOpcSignatureReference {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1b47005e_3011_4edc_be6f_0f65e5ab0342);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignatureReference_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, referenceid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, referenceuri: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetUri: usize,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetTransformMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transformmethod: *mut OPC_CANONICALIZATION_METHOD) -> ::windows_core::HRESULT,
    pub GetDigestMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, digestmethod: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetDigestValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOpcSignatureReferenceEnumerator(::windows_core::IUnknown);
impl IOpcSignatureReferenceEnumerator {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MoveNext)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MovePrevious(&self) -> ::windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MovePrevious)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCurrent(&self) -> ::windows_core::Result<IOpcSignatureReference> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCurrent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IOpcSignatureReferenceEnumerator> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IOpcSignatureReferenceEnumerator, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOpcSignatureReferenceEnumerator {
    type Vtable = IOpcSignatureReferenceEnumerator_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOpcSignatureReferenceEnumerator {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcfa59a45_28b1_4868_969e_fa8097fdc12a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignatureReferenceEnumerator_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MovePrevious: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MovePrevious: usize,
    pub GetCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, copy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOpcSignatureReferenceSet(::windows_core::IUnknown);
impl IOpcSignatureReferenceSet {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Create<P0, P1, P2, P3>(&self, referenceuri: P0, referenceid: P1, r#type: P2, digestmethod: P3, transformmethod: OPC_CANONICALIZATION_METHOD) -> ::windows_core::Result<IOpcSignatureReference>
    where
        P0: ::windows_core::IntoParam<super::super::super::System::Com::IUri>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Create)(::windows_core::Interface::as_raw(self), referenceuri.into_param().abi(), referenceid.into_param().abi(), r#type.into_param().abi(), digestmethod.into_param().abi(), transformmethod, &mut result__).from_abi(result__)
    }
    pub unsafe fn Delete<P0>(&self, reference: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IOpcSignatureReference>,
    {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self), reference.into_param().abi()).ok()
    }
    pub unsafe fn GetEnumerator(&self) -> ::windows_core::Result<IOpcSignatureReferenceEnumerator> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetEnumerator)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IOpcSignatureReferenceSet, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOpcSignatureReferenceSet {
    type Vtable = IOpcSignatureReferenceSet_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOpcSignatureReferenceSet {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf3b02d31_ab12_42dd_9e2f_2b16761c3c1e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignatureReferenceSet_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, referenceuri: *mut ::core::ffi::c_void, referenceid: ::windows_core::PCWSTR, r#type: ::windows_core::PCWSTR, digestmethod: ::windows_core::PCWSTR, transformmethod: OPC_CANONICALIZATION_METHOD, reference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Create: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reference: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, referenceenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOpcSignatureRelationshipReference(::windows_core::IUnknown);
impl IOpcSignatureRelationshipReference {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSourceUri(&self) -> ::windows_core::Result<IOpcUri> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSourceUri)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDigestMethod(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDigestMethod)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDigestValue(&self, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDigestValue)(::windows_core::Interface::as_raw(self), digestvalue, count).ok()
    }
    pub unsafe fn GetTransformMethod(&self) -> ::windows_core::Result<OPC_CANONICALIZATION_METHOD> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTransformMethod)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRelationshipSigningOption(&self) -> ::windows_core::Result<OPC_RELATIONSHIPS_SIGNING_OPTION> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRelationshipSigningOption)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRelationshipSelectorEnumerator(&self) -> ::windows_core::Result<IOpcRelationshipSelectorEnumerator> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRelationshipSelectorEnumerator)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IOpcSignatureRelationshipReference, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOpcSignatureRelationshipReference {
    type Vtable = IOpcSignatureRelationshipReference_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOpcSignatureRelationshipReference {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x57babac6_9d4a_4e50_8b86_e5d4051eae7c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignatureRelationshipReference_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSourceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceuri: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSourceUri: usize,
    pub GetDigestMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, digestmethod: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetDigestValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows_core::HRESULT,
    pub GetTransformMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transformmethod: *mut OPC_CANONICALIZATION_METHOD) -> ::windows_core::HRESULT,
    pub GetRelationshipSigningOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relationshipsigningoption: *mut OPC_RELATIONSHIPS_SIGNING_OPTION) -> ::windows_core::HRESULT,
    pub GetRelationshipSelectorEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selectorenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOpcSignatureRelationshipReferenceEnumerator(::windows_core::IUnknown);
impl IOpcSignatureRelationshipReferenceEnumerator {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MoveNext)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MovePrevious(&self) -> ::windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MovePrevious)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCurrent(&self) -> ::windows_core::Result<IOpcSignatureRelationshipReference> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCurrent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IOpcSignatureRelationshipReferenceEnumerator> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IOpcSignatureRelationshipReferenceEnumerator, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOpcSignatureRelationshipReferenceEnumerator {
    type Vtable = IOpcSignatureRelationshipReferenceEnumerator_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOpcSignatureRelationshipReferenceEnumerator {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x773ba3e4_f021_48e4_aa04_9816db5d3495);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignatureRelationshipReferenceEnumerator_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MovePrevious: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MovePrevious: usize,
    pub GetCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relationshipreference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, copy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOpcSignatureRelationshipReferenceSet(::windows_core::IUnknown);
impl IOpcSignatureRelationshipReferenceSet {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Create<P0, P1, P2>(&self, sourceuri: P0, digestmethod: P1, relationshipsigningoption: OPC_RELATIONSHIPS_SIGNING_OPTION, selectorset: P2, transformmethod: OPC_CANONICALIZATION_METHOD) -> ::windows_core::Result<IOpcSignatureRelationshipReference>
    where
        P0: ::windows_core::IntoParam<IOpcUri>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<IOpcRelationshipSelectorSet>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Create)(::windows_core::Interface::as_raw(self), sourceuri.into_param().abi(), digestmethod.into_param().abi(), relationshipsigningoption, selectorset.into_param().abi(), transformmethod, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateRelationshipSelectorSet(&self) -> ::windows_core::Result<IOpcRelationshipSelectorSet> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateRelationshipSelectorSet)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Delete<P0>(&self, relationshipreference: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IOpcSignatureRelationshipReference>,
    {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self), relationshipreference.into_param().abi()).ok()
    }
    pub unsafe fn GetEnumerator(&self) -> ::windows_core::Result<IOpcSignatureRelationshipReferenceEnumerator> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetEnumerator)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IOpcSignatureRelationshipReferenceSet, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOpcSignatureRelationshipReferenceSet {
    type Vtable = IOpcSignatureRelationshipReferenceSet_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOpcSignatureRelationshipReferenceSet {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9f863ca5_3631_404c_828d_807e0715069b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignatureRelationshipReferenceSet_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceuri: *mut ::core::ffi::c_void, digestmethod: ::windows_core::PCWSTR, relationshipsigningoption: OPC_RELATIONSHIPS_SIGNING_OPTION, selectorset: *mut ::core::ffi::c_void, transformmethod: OPC_CANONICALIZATION_METHOD, relationshipreference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Create: usize,
    pub CreateRelationshipSelectorSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selectorset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relationshipreference: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relationshipreferenceenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOpcSigningOptions(::windows_core::IUnknown);
impl IOpcSigningOptions {
    pub unsafe fn GetSignatureId(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSignatureId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSignatureId<P0>(&self, signatureid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetSignatureId)(::windows_core::Interface::as_raw(self), signatureid.into_param().abi()).ok()
    }
    pub unsafe fn GetSignatureMethod(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSignatureMethod)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSignatureMethod<P0>(&self, signaturemethod: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetSignatureMethod)(::windows_core::Interface::as_raw(self), signaturemethod.into_param().abi()).ok()
    }
    pub unsafe fn GetDefaultDigestMethod(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDefaultDigestMethod)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDefaultDigestMethod<P0>(&self, digestmethod: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDefaultDigestMethod)(::windows_core::Interface::as_raw(self), digestmethod.into_param().abi()).ok()
    }
    pub unsafe fn GetCertificateEmbeddingOption(&self) -> ::windows_core::Result<OPC_CERTIFICATE_EMBEDDING_OPTION> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCertificateEmbeddingOption)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCertificateEmbeddingOption(&self, embeddingoption: OPC_CERTIFICATE_EMBEDDING_OPTION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCertificateEmbeddingOption)(::windows_core::Interface::as_raw(self), embeddingoption).ok()
    }
    pub unsafe fn GetTimeFormat(&self) -> ::windows_core::Result<OPC_SIGNATURE_TIME_FORMAT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTimeFormat)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTimeFormat(&self, timeformat: OPC_SIGNATURE_TIME_FORMAT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTimeFormat)(::windows_core::Interface::as_raw(self), timeformat).ok()
    }
    pub unsafe fn GetSignaturePartReferenceSet(&self) -> ::windows_core::Result<IOpcSignaturePartReferenceSet> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSignaturePartReferenceSet)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSignatureRelationshipReferenceSet(&self) -> ::windows_core::Result<IOpcSignatureRelationshipReferenceSet> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSignatureRelationshipReferenceSet)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCustomObjectSet(&self) -> ::windows_core::Result<IOpcSignatureCustomObjectSet> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCustomObjectSet)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCustomReferenceSet(&self) -> ::windows_core::Result<IOpcSignatureReferenceSet> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCustomReferenceSet)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCertificateSet(&self) -> ::windows_core::Result<IOpcCertificateSet> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCertificateSet)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSignaturePartName(&self) -> ::windows_core::Result<IOpcPartUri> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSignaturePartName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSignaturePartName<P0>(&self, signaturepartname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IOpcPartUri>,
    {
        (::windows_core::Interface::vtable(self).SetSignaturePartName)(::windows_core::Interface::as_raw(self), signaturepartname.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IOpcSigningOptions, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOpcSigningOptions {
    type Vtable = IOpcSigningOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOpcSigningOptions {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x50d2d6a5_7aeb_46c0_b241_43ab0e9b407e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSigningOptions_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetSignatureId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signatureid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetSignatureId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signatureid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetSignatureMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturemethod: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetSignatureMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturemethod: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetDefaultDigestMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, digestmethod: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetDefaultDigestMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, digestmethod: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetCertificateEmbeddingOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, embeddingoption: *mut OPC_CERTIFICATE_EMBEDDING_OPTION) -> ::windows_core::HRESULT,
    pub SetCertificateEmbeddingOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, embeddingoption: OPC_CERTIFICATE_EMBEDDING_OPTION) -> ::windows_core::HRESULT,
    pub GetTimeFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeformat: *mut OPC_SIGNATURE_TIME_FORMAT) -> ::windows_core::HRESULT,
    pub SetTimeFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeformat: OPC_SIGNATURE_TIME_FORMAT) -> ::windows_core::HRESULT,
    pub GetSignaturePartReferenceSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partreferenceset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetSignatureRelationshipReferenceSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relationshipreferenceset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetCustomObjectSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customobjectset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetCustomReferenceSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customreferenceset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetCertificateSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, certificateset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSignaturePartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturepartname: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSignaturePartName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSignaturePartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturepartname: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSignaturePartName: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOpcUri(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IOpcUri {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPropertyBSTR(&self, uriprop: super::super::super::System::Com::Uri_PROPERTY, pbstrproperty: *mut ::windows_core::BSTR, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetPropertyBSTR)(::windows_core::Interface::as_raw(self), uriprop, ::core::mem::transmute(pbstrproperty), dwflags).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPropertyLength(&self, uriprop: super::super::super::System::Com::Uri_PROPERTY, pcchproperty: *mut u32, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetPropertyLength)(::windows_core::Interface::as_raw(self), uriprop, pcchproperty, dwflags).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPropertyDWORD(&self, uriprop: super::super::super::System::Com::Uri_PROPERTY, pdwproperty: *mut u32, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetPropertyDWORD)(::windows_core::Interface::as_raw(self), uriprop, pdwproperty, dwflags).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn HasProperty(&self, uriprop: super::super::super::System::Com::Uri_PROPERTY) -> ::windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.HasProperty)(::windows_core::Interface::as_raw(self), uriprop, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetAbsoluteUri(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetAbsoluteUri)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetAuthority(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetAuthority)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDisplayUri(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDisplayUri)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDomain(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDomain)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetExtension(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetExtension)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFragment(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetFragment)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetHost(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetHost)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPassword(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPassword)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPathAndQuery(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPathAndQuery)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetQuery(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetQuery)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRawUri(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetRawUri)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSchemeName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSchemeName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetUserInfo(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetUserInfo)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetUserName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetUserName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetHostType(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetHostType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPort(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPort)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetScheme(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetScheme)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetZone(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetZone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetProperties(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetProperties)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn IsEqual<P0>(&self, puri: P0) -> ::windows_core::Result<super::super::super::Foundation::BOOL>
    where
        P0: ::windows_core::IntoParam<super::super::super::System::Com::IUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsEqual)(::windows_core::Interface::as_raw(self), puri.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRelationshipsPartUri(&self) -> ::windows_core::Result<IOpcPartUri> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRelationshipsPartUri)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRelativeUri<P0>(&self, targetparturi: P0) -> ::windows_core::Result<super::super::super::System::Com::IUri>
    where
        P0: ::windows_core::IntoParam<IOpcPartUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRelativeUri)(::windows_core::Interface::as_raw(self), targetparturi.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CombinePartUri<P0>(&self, relativeuri: P0) -> ::windows_core::Result<IOpcPartUri>
    where
        P0: ::windows_core::IntoParam<super::super::super::System::Com::IUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CombinePartUri)(::windows_core::Interface::as_raw(self), relativeuri.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IOpcUri, ::windows_core::IUnknown, super::super::super::System::Com::IUri);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IOpcUri {
    type Vtable = IOpcUri_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IOpcUri {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbc9c1b9b_d62c_49eb_aef0_3b4e0b28ebed);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IOpcUri_Vtbl {
    pub base__: super::super::super::System::Com::IUri_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRelationshipsPartUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relationshipparturi: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRelationshipsPartUri: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRelativeUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetparturi: *mut ::core::ffi::c_void, relativeuri: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRelativeUri: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CombinePartUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relativeuri: *mut ::core::ffi::c_void, combineduri: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CombinePartUri: usize,
}
pub const OPC_CACHE_ON_ACCESS: OPC_READ_FLAGS = OPC_READ_FLAGS(2i32);
pub const OPC_CANONICALIZATION_C14N: OPC_CANONICALIZATION_METHOD = OPC_CANONICALIZATION_METHOD(1i32);
pub const OPC_CANONICALIZATION_C14N_WITH_COMMENTS: OPC_CANONICALIZATION_METHOD = OPC_CANONICALIZATION_METHOD(2i32);
pub const OPC_CANONICALIZATION_NONE: OPC_CANONICALIZATION_METHOD = OPC_CANONICALIZATION_METHOD(0i32);
pub const OPC_CERTIFICATE_IN_CERTIFICATE_PART: OPC_CERTIFICATE_EMBEDDING_OPTION = OPC_CERTIFICATE_EMBEDDING_OPTION(0i32);
pub const OPC_CERTIFICATE_IN_SIGNATURE_PART: OPC_CERTIFICATE_EMBEDDING_OPTION = OPC_CERTIFICATE_EMBEDDING_OPTION(1i32);
pub const OPC_CERTIFICATE_NOT_EMBEDDED: OPC_CERTIFICATE_EMBEDDING_OPTION = OPC_CERTIFICATE_EMBEDDING_OPTION(2i32);
pub const OPC_COMPRESSION_FAST: OPC_COMPRESSION_OPTIONS = OPC_COMPRESSION_OPTIONS(2i32);
pub const OPC_COMPRESSION_MAXIMUM: OPC_COMPRESSION_OPTIONS = OPC_COMPRESSION_OPTIONS(1i32);
pub const OPC_COMPRESSION_NONE: OPC_COMPRESSION_OPTIONS = OPC_COMPRESSION_OPTIONS(-1i32);
pub const OPC_COMPRESSION_NORMAL: OPC_COMPRESSION_OPTIONS = OPC_COMPRESSION_OPTIONS(0i32);
pub const OPC_COMPRESSION_SUPERFAST: OPC_COMPRESSION_OPTIONS = OPC_COMPRESSION_OPTIONS(3i32);
pub const OPC_E_CONFLICTING_SETTINGS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175212i32);
pub const OPC_E_COULD_NOT_RECOVER: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175154i32);
pub const OPC_E_DS_DEFAULT_DIGEST_METHOD_NOT_SET: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175161i32);
pub const OPC_E_DS_DIGEST_VALUE_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175206i32);
pub const OPC_E_DS_DUPLICATE_PACKAGE_OBJECT_REFERENCES: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175187i32);
pub const OPC_E_DS_DUPLICATE_SIGNATURE_ORIGIN_RELATIONSHIP: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175205i32);
pub const OPC_E_DS_DUPLICATE_SIGNATURE_PROPERTY_ELEMENT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175192i32);
pub const OPC_E_DS_EXTERNAL_SIGNATURE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175202i32);
pub const OPC_E_DS_EXTERNAL_SIGNATURE_REFERENCE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175185i32);
pub const OPC_E_DS_INVALID_CANONICALIZATION_METHOD: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175198i32);
pub const OPC_E_DS_INVALID_CERTIFICATE_RELATIONSHIP: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175203i32);
pub const OPC_E_DS_INVALID_OPC_SIGNATURE_TIME_FORMAT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175196i32);
pub const OPC_E_DS_INVALID_RELATIONSHIPS_SIGNING_OPTION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175197i32);
pub const OPC_E_DS_INVALID_RELATIONSHIP_TRANSFORM_XML: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175199i32);
pub const OPC_E_DS_INVALID_SIGNATURE_COUNT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175189i32);
pub const OPC_E_DS_INVALID_SIGNATURE_ORIGIN_RELATIONSHIP: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175204i32);
pub const OPC_E_DS_INVALID_SIGNATURE_XML: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175190i32);
pub const OPC_E_DS_MISSING_CANONICALIZATION_TRANSFORM: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175182i32);
pub const OPC_E_DS_MISSING_CERTIFICATE_PART: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175146i32);
pub const OPC_E_DS_MISSING_PACKAGE_OBJECT_REFERENCE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175186i32);
pub const OPC_E_DS_MISSING_SIGNATURE_ALGORITHM: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175188i32);
pub const OPC_E_DS_MISSING_SIGNATURE_ORIGIN_PART: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175201i32);
pub const OPC_E_DS_MISSING_SIGNATURE_PART: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175200i32);
pub const OPC_E_DS_MISSING_SIGNATURE_PROPERTIES_ELEMENT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175194i32);
pub const OPC_E_DS_MISSING_SIGNATURE_PROPERTY_ELEMENT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175193i32);
pub const OPC_E_DS_MISSING_SIGNATURE_TIME_PROPERTY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175191i32);
pub const OPC_E_DS_MULTIPLE_RELATIONSHIP_TRANSFORMS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175183i32);
pub const OPC_E_DS_PACKAGE_REFERENCE_URI_RESERVED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175195i32);
pub const OPC_E_DS_REFERENCE_MISSING_CONTENT_TYPE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175184i32);
pub const OPC_E_DS_SIGNATURE_CORRUPT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175207i32);
pub const OPC_E_DS_SIGNATURE_METHOD_NOT_SET: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175162i32);
pub const OPC_E_DS_SIGNATURE_ORIGIN_EXISTS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175148i32);
pub const OPC_E_DS_SIGNATURE_PROPERTY_MISSING_TARGET: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175163i32);
pub const OPC_E_DS_SIGNATURE_REFERENCE_MISSING_URI: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175165i32);
pub const OPC_E_DS_UNSIGNED_PACKAGE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175147i32);
pub const OPC_E_DUPLICATE_DEFAULT_EXTENSION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175217i32);
pub const OPC_E_DUPLICATE_OVERRIDE_PART: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175219i32);
pub const OPC_E_DUPLICATE_PART: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175221i32);
pub const OPC_E_DUPLICATE_PIECE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175211i32);
pub const OPC_E_DUPLICATE_RELATIONSHIP: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175213i32);
pub const OPC_E_ENUM_CANNOT_MOVE_NEXT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175151i32);
pub const OPC_E_ENUM_CANNOT_MOVE_PREVIOUS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175150i32);
pub const OPC_E_ENUM_COLLECTION_CHANGED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175152i32);
pub const OPC_E_ENUM_INVALID_POSITION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175149i32);
pub const OPC_E_INVALID_CONTENT_TYPE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175164i32);
pub const OPC_E_INVALID_CONTENT_TYPE_XML: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175226i32);
pub const OPC_E_INVALID_DEFAULT_EXTENSION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175218i32);
pub const OPC_E_INVALID_OVERRIDE_PART_NAME: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175220i32);
pub const OPC_E_INVALID_PIECE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175210i32);
pub const OPC_E_INVALID_RELATIONSHIP_ID: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175216i32);
pub const OPC_E_INVALID_RELATIONSHIP_TARGET: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175214i32);
pub const OPC_E_INVALID_RELATIONSHIP_TARGET_MODE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175155i32);
pub const OPC_E_INVALID_RELATIONSHIP_TYPE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175215i32);
pub const OPC_E_INVALID_RELS_XML: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175222i32);
pub const OPC_E_INVALID_XML_ENCODING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175166i32);
pub const OPC_E_MC_INCONSISTENT_PRESERVE_ATTRIBUTES: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175157i32);
pub const OPC_E_MC_INCONSISTENT_PRESERVE_ELEMENTS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175156i32);
pub const OPC_E_MC_INCONSISTENT_PROCESS_CONTENT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175158i32);
pub const OPC_E_MC_INVALID_ATTRIBUTES_ON_IGNORABLE_ELEMENT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175168i32);
pub const OPC_E_MC_INVALID_ENUM_TYPE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175172i32);
pub const OPC_E_MC_INVALID_PREFIX_LIST: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175177i32);
pub const OPC_E_MC_INVALID_QNAME_LIST: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175176i32);
pub const OPC_E_MC_INVALID_XMLNS_ATTRIBUTE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175167i32);
pub const OPC_E_MC_MISSING_CHOICE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175173i32);
pub const OPC_E_MC_MISSING_REQUIRES_ATTR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175179i32);
pub const OPC_E_MC_MULTIPLE_FALLBACK_ELEMENTS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175159i32);
pub const OPC_E_MC_NESTED_ALTERNATE_CONTENT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175175i32);
pub const OPC_E_MC_UNEXPECTED_ATTR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175178i32);
pub const OPC_E_MC_UNEXPECTED_CHOICE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175174i32);
pub const OPC_E_MC_UNEXPECTED_ELEMENT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175181i32);
pub const OPC_E_MC_UNEXPECTED_REQUIRES_ATTR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175180i32);
pub const OPC_E_MC_UNKNOWN_NAMESPACE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175170i32);
pub const OPC_E_MC_UNKNOWN_PREFIX: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175169i32);
pub const OPC_E_MISSING_CONTENT_TYPES: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175225i32);
pub const OPC_E_MISSING_PIECE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175209i32);
pub const OPC_E_NONCONFORMING_CONTENT_TYPES_XML: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175224i32);
pub const OPC_E_NONCONFORMING_RELS_XML: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175223i32);
pub const OPC_E_NONCONFORMING_URI: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175231i32);
pub const OPC_E_NO_SUCH_PART: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175208i32);
pub const OPC_E_NO_SUCH_RELATIONSHIP: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175160i32);
pub const OPC_E_NO_SUCH_SETTINGS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175145i32);
pub const OPC_E_PART_CANNOT_BE_DIRECTORY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175228i32);
pub const OPC_E_RELATIONSHIP_URI_REQUIRED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175229i32);
pub const OPC_E_RELATIVE_URI_REQUIRED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175230i32);
pub const OPC_E_UNEXPECTED_CONTENT_TYPE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175227i32);
pub const OPC_E_UNSUPPORTED_PACKAGE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142175153i32);
pub const OPC_E_ZIP_CENTRAL_DIRECTORY_TOO_LARGE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142171127i32);
pub const OPC_E_ZIP_COMMENT_TOO_LARGE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142171124i32);
pub const OPC_E_ZIP_COMPRESSION_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142171133i32);
pub const OPC_E_ZIP_CORRUPTED_ARCHIVE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142171134i32);
pub const OPC_E_ZIP_DECOMPRESSION_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142171132i32);
pub const OPC_E_ZIP_DUPLICATE_NAME: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142171125i32);
pub const OPC_E_ZIP_EXTRA_FIELDS_TOO_LARGE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142171123i32);
pub const OPC_E_ZIP_FILE_HEADER_TOO_LARGE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142171122i32);
pub const OPC_E_ZIP_INCONSISTENT_DIRECTORY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142171130i32);
pub const OPC_E_ZIP_INCONSISTENT_FILEITEM: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142171131i32);
pub const OPC_E_ZIP_INCORRECT_DATA_SIZE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142171135i32);
pub const OPC_E_ZIP_MISSING_DATA_DESCRIPTOR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142171129i32);
pub const OPC_E_ZIP_MISSING_END_OF_CENTRAL_DIRECTORY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142171121i32);
pub const OPC_E_ZIP_NAME_TOO_LARGE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142171126i32);
pub const OPC_E_ZIP_REQUIRES_64_BIT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142171120i32);
pub const OPC_E_ZIP_UNSUPPORTEDARCHIVE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142171128i32);
pub const OPC_READ_DEFAULT: OPC_READ_FLAGS = OPC_READ_FLAGS(0i32);
pub const OPC_RELATIONSHIP_SELECT_BY_ID: OPC_RELATIONSHIP_SELECTOR = OPC_RELATIONSHIP_SELECTOR(0i32);
pub const OPC_RELATIONSHIP_SELECT_BY_TYPE: OPC_RELATIONSHIP_SELECTOR = OPC_RELATIONSHIP_SELECTOR(1i32);
pub const OPC_RELATIONSHIP_SIGN_PART: OPC_RELATIONSHIPS_SIGNING_OPTION = OPC_RELATIONSHIPS_SIGNING_OPTION(1i32);
pub const OPC_RELATIONSHIP_SIGN_USING_SELECTORS: OPC_RELATIONSHIPS_SIGNING_OPTION = OPC_RELATIONSHIPS_SIGNING_OPTION(0i32);
pub const OPC_SIGNATURE_INVALID: OPC_SIGNATURE_VALIDATION_RESULT = OPC_SIGNATURE_VALIDATION_RESULT(-1i32);
pub const OPC_SIGNATURE_TIME_FORMAT_DAYS: OPC_SIGNATURE_TIME_FORMAT = OPC_SIGNATURE_TIME_FORMAT(3i32);
pub const OPC_SIGNATURE_TIME_FORMAT_MILLISECONDS: OPC_SIGNATURE_TIME_FORMAT = OPC_SIGNATURE_TIME_FORMAT(0i32);
pub const OPC_SIGNATURE_TIME_FORMAT_MINUTES: OPC_SIGNATURE_TIME_FORMAT = OPC_SIGNATURE_TIME_FORMAT(2i32);
pub const OPC_SIGNATURE_TIME_FORMAT_MONTHS: OPC_SIGNATURE_TIME_FORMAT = OPC_SIGNATURE_TIME_FORMAT(4i32);
pub const OPC_SIGNATURE_TIME_FORMAT_SECONDS: OPC_SIGNATURE_TIME_FORMAT = OPC_SIGNATURE_TIME_FORMAT(1i32);
pub const OPC_SIGNATURE_TIME_FORMAT_YEARS: OPC_SIGNATURE_TIME_FORMAT = OPC_SIGNATURE_TIME_FORMAT(5i32);
pub const OPC_SIGNATURE_VALID: OPC_SIGNATURE_VALIDATION_RESULT = OPC_SIGNATURE_VALIDATION_RESULT(0i32);
pub const OPC_STREAM_IO_READ: OPC_STREAM_IO_MODE = OPC_STREAM_IO_MODE(1i32);
pub const OPC_STREAM_IO_WRITE: OPC_STREAM_IO_MODE = OPC_STREAM_IO_MODE(2i32);
pub const OPC_URI_TARGET_MODE_EXTERNAL: OPC_URI_TARGET_MODE = OPC_URI_TARGET_MODE(1i32);
pub const OPC_URI_TARGET_MODE_INTERNAL: OPC_URI_TARGET_MODE = OPC_URI_TARGET_MODE(0i32);
pub const OPC_VALIDATE_ON_LOAD: OPC_READ_FLAGS = OPC_READ_FLAGS(1i32);
pub const OPC_WRITE_DEFAULT: OPC_WRITE_FLAGS = OPC_WRITE_FLAGS(0i32);
pub const OPC_WRITE_FORCE_ZIP32: OPC_WRITE_FLAGS = OPC_WRITE_FLAGS(1i32);
pub const OpcFactory: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6b2d6ba0_9f3e_4f27_920b_313cc426a39e);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OPC_CANONICALIZATION_METHOD(pub i32);
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
impl ::windows_core::TypeKind for OPC_CANONICALIZATION_METHOD {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for OPC_CANONICALIZATION_METHOD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPC_CANONICALIZATION_METHOD").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OPC_CERTIFICATE_EMBEDDING_OPTION(pub i32);
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
impl ::windows_core::TypeKind for OPC_CERTIFICATE_EMBEDDING_OPTION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for OPC_CERTIFICATE_EMBEDDING_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPC_CERTIFICATE_EMBEDDING_OPTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OPC_COMPRESSION_OPTIONS(pub i32);
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
impl ::windows_core::TypeKind for OPC_COMPRESSION_OPTIONS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for OPC_COMPRESSION_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPC_COMPRESSION_OPTIONS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OPC_READ_FLAGS(pub i32);
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
impl ::windows_core::TypeKind for OPC_READ_FLAGS {
    type TypeKind = ::windows_core::CopyType;
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
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OPC_RELATIONSHIPS_SIGNING_OPTION(pub i32);
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
impl ::windows_core::TypeKind for OPC_RELATIONSHIPS_SIGNING_OPTION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for OPC_RELATIONSHIPS_SIGNING_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPC_RELATIONSHIPS_SIGNING_OPTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OPC_RELATIONSHIP_SELECTOR(pub i32);
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
impl ::windows_core::TypeKind for OPC_RELATIONSHIP_SELECTOR {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for OPC_RELATIONSHIP_SELECTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPC_RELATIONSHIP_SELECTOR").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OPC_SIGNATURE_TIME_FORMAT(pub i32);
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
impl ::windows_core::TypeKind for OPC_SIGNATURE_TIME_FORMAT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for OPC_SIGNATURE_TIME_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPC_SIGNATURE_TIME_FORMAT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OPC_SIGNATURE_VALIDATION_RESULT(pub i32);
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
impl ::windows_core::TypeKind for OPC_SIGNATURE_VALIDATION_RESULT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for OPC_SIGNATURE_VALIDATION_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPC_SIGNATURE_VALIDATION_RESULT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OPC_STREAM_IO_MODE(pub i32);
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
impl ::windows_core::TypeKind for OPC_STREAM_IO_MODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for OPC_STREAM_IO_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPC_STREAM_IO_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OPC_URI_TARGET_MODE(pub i32);
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
impl ::windows_core::TypeKind for OPC_URI_TARGET_MODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for OPC_URI_TARGET_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPC_URI_TARGET_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OPC_WRITE_FLAGS(pub i32);
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
impl ::windows_core::TypeKind for OPC_WRITE_FLAGS {
    type TypeKind = ::windows_core::CopyType;
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
