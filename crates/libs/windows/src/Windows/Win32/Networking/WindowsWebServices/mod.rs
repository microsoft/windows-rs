#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
pub struct IContentPrefetcherTaskTrigger(::windows::core::IUnknown);
impl IContentPrefetcherTaskTrigger {
    #[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
    pub unsafe fn TriggerContentPrefetcherTask<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, packagefullname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TriggerContentPrefetcherTask)(::windows::core::Interface::as_raw(self), packagefullname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
    pub unsafe fn IsRegisteredForContentPrefetch<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, packagefullname: Param0) -> ::windows::core::Result<u8> {
        let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
        (::windows::core::Interface::vtable(self).IsRegisteredForContentPrefetch)(::windows::core::Interface::as_raw(self), packagefullname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u8>(result__)
    }
}
impl ::core::convert::From<IContentPrefetcherTaskTrigger> for ::windows::core::IUnknown {
    fn from(value: IContentPrefetcherTaskTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContentPrefetcherTaskTrigger> for ::windows::core::IUnknown {
    fn from(value: &IContentPrefetcherTaskTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IContentPrefetcherTaskTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IContentPrefetcherTaskTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IContentPrefetcherTaskTrigger> for ::windows::core::IInspectable {
    fn from(value: IContentPrefetcherTaskTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContentPrefetcherTaskTrigger> for ::windows::core::IInspectable {
    fn from(value: &IContentPrefetcherTaskTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IContentPrefetcherTaskTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IContentPrefetcherTaskTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IContentPrefetcherTaskTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContentPrefetcherTaskTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContentPrefetcherTaskTrigger {}
impl ::core::fmt::Debug for IContentPrefetcherTaskTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContentPrefetcherTaskTrigger").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IContentPrefetcherTaskTrigger {
    type Vtable = IContentPrefetcherTaskTrigger_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b35a14a_6094_4799_a60e_e474e15d4dc9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentPrefetcherTaskTrigger_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub TriggerContentPrefetcherTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefullname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub IsRegisteredForContentPrefetch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefullname: ::windows::core::PCWSTR, isregistered: *mut u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_API_CURRENT_VERSION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_API_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_API_VERSION_2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_API_VERSION_3: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WEBAUTHN_ASSERTION {
    pub dwVersion: u32,
    pub cbAuthenticatorData: u32,
    pub pbAuthenticatorData: *mut u8,
    pub cbSignature: u32,
    pub pbSignature: *mut u8,
    pub Credential: WEBAUTHN_CREDENTIAL,
    pub cbUserId: u32,
    pub pbUserId: *mut u8,
    pub Extensions: WEBAUTHN_EXTENSIONS,
    pub cbCredLargeBlob: u32,
    pub pbCredLargeBlob: *mut u8,
    pub dwCredLargeBlobStatus: u32,
}
impl ::core::marker::Copy for WEBAUTHN_ASSERTION {}
impl ::core::clone::Clone for WEBAUTHN_ASSERTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WEBAUTHN_ASSERTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_ASSERTION")
            .field("dwVersion", &self.dwVersion)
            .field("cbAuthenticatorData", &self.cbAuthenticatorData)
            .field("pbAuthenticatorData", &self.pbAuthenticatorData)
            .field("cbSignature", &self.cbSignature)
            .field("pbSignature", &self.pbSignature)
            .field("Credential", &self.Credential)
            .field("cbUserId", &self.cbUserId)
            .field("pbUserId", &self.pbUserId)
            .field("Extensions", &self.Extensions)
            .field("cbCredLargeBlob", &self.cbCredLargeBlob)
            .field("pbCredLargeBlob", &self.pbCredLargeBlob)
            .field("dwCredLargeBlobStatus", &self.dwCredLargeBlobStatus)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for WEBAUTHN_ASSERTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WEBAUTHN_ASSERTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WEBAUTHN_ASSERTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WEBAUTHN_ASSERTION {}
impl ::core::default::Default for WEBAUTHN_ASSERTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_ASSERTION_CURRENT_VERSION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_ASSERTION_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_ASSERTION_VERSION_2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_ATTESTATION_CONVEYANCE_PREFERENCE_ANY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_ATTESTATION_CONVEYANCE_PREFERENCE_DIRECT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_ATTESTATION_CONVEYANCE_PREFERENCE_INDIRECT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_ATTESTATION_CONVEYANCE_PREFERENCE_NONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_ATTESTATION_DECODE_COMMON: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_ATTESTATION_DECODE_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_ATTESTATION_TYPE_NONE: &str = "none";
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_ATTESTATION_TYPE_PACKED: &str = "packed";
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_ATTESTATION_TYPE_TPM: &str = "tpm";
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_ATTESTATION_TYPE_U2F: &str = "fido-u2f";
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_ATTESTATION_VER_TPM_2_0: &str = "2.0";
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_AUTHENTICATOR_ATTACHMENT_ANY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_AUTHENTICATOR_ATTACHMENT_CROSS_PLATFORM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_AUTHENTICATOR_ATTACHMENT_CROSS_PLATFORM_U2F_V2: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_AUTHENTICATOR_ATTACHMENT_PLATFORM: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS {
    pub dwVersion: u32,
    pub dwTimeoutMilliseconds: u32,
    pub CredentialList: WEBAUTHN_CREDENTIALS,
    pub Extensions: WEBAUTHN_EXTENSIONS,
    pub dwAuthenticatorAttachment: u32,
    pub dwUserVerificationRequirement: u32,
    pub dwFlags: u32,
    pub pwszU2fAppId: ::windows::core::PCWSTR,
    pub pbU2fAppId: *mut super::super::Foundation::BOOL,
    pub pCancellationId: *mut ::windows::core::GUID,
    pub pAllowCredentialList: *mut WEBAUTHN_CREDENTIAL_LIST,
    pub dwCredLargeBlobOperation: u32,
    pub cbCredLargeBlob: u32,
    pub pbCredLargeBlob: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS")
            .field("dwVersion", &self.dwVersion)
            .field("dwTimeoutMilliseconds", &self.dwTimeoutMilliseconds)
            .field("CredentialList", &self.CredentialList)
            .field("Extensions", &self.Extensions)
            .field("dwAuthenticatorAttachment", &self.dwAuthenticatorAttachment)
            .field("dwUserVerificationRequirement", &self.dwUserVerificationRequirement)
            .field("dwFlags", &self.dwFlags)
            .field("pwszU2fAppId", &self.pwszU2fAppId)
            .field("pbU2fAppId", &self.pbU2fAppId)
            .field("pCancellationId", &self.pCancellationId)
            .field("pAllowCredentialList", &self.pAllowCredentialList)
            .field("dwCredLargeBlobOperation", &self.dwCredLargeBlobOperation)
            .field("cbCredLargeBlob", &self.cbCredLargeBlob)
            .field("pbCredLargeBlob", &self.pbCredLargeBlob)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_CURRENT_VERSION: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_3: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_4: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_5: u32 = 5u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS {
    pub dwVersion: u32,
    pub dwTimeoutMilliseconds: u32,
    pub CredentialList: WEBAUTHN_CREDENTIALS,
    pub Extensions: WEBAUTHN_EXTENSIONS,
    pub dwAuthenticatorAttachment: u32,
    pub bRequireResidentKey: super::super::Foundation::BOOL,
    pub dwUserVerificationRequirement: u32,
    pub dwAttestationConveyancePreference: u32,
    pub dwFlags: u32,
    pub pCancellationId: *mut ::windows::core::GUID,
    pub pExcludeCredentialList: *mut WEBAUTHN_CREDENTIAL_LIST,
    pub dwEnterpriseAttestation: u32,
    pub dwLargeBlobSupport: u32,
    pub bPreferResidentKey: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS")
            .field("dwVersion", &self.dwVersion)
            .field("dwTimeoutMilliseconds", &self.dwTimeoutMilliseconds)
            .field("CredentialList", &self.CredentialList)
            .field("Extensions", &self.Extensions)
            .field("dwAuthenticatorAttachment", &self.dwAuthenticatorAttachment)
            .field("bRequireResidentKey", &self.bRequireResidentKey)
            .field("dwUserVerificationRequirement", &self.dwUserVerificationRequirement)
            .field("dwAttestationConveyancePreference", &self.dwAttestationConveyancePreference)
            .field("dwFlags", &self.dwFlags)
            .field("pCancellationId", &self.pCancellationId)
            .field("pExcludeCredentialList", &self.pExcludeCredentialList)
            .field("dwEnterpriseAttestation", &self.dwEnterpriseAttestation)
            .field("dwLargeBlobSupport", &self.dwLargeBlobSupport)
            .field("bPreferResidentKey", &self.bPreferResidentKey)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_CURRENT_VERSION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_3: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_4: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WEBAUTHN_CLIENT_DATA {
    pub dwVersion: u32,
    pub cbClientDataJSON: u32,
    pub pbClientDataJSON: *mut u8,
    pub pwszHashAlgId: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for WEBAUTHN_CLIENT_DATA {}
impl ::core::clone::Clone for WEBAUTHN_CLIENT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WEBAUTHN_CLIENT_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_CLIENT_DATA").field("dwVersion", &self.dwVersion).field("cbClientDataJSON", &self.cbClientDataJSON).field("pbClientDataJSON", &self.pbClientDataJSON).field("pwszHashAlgId", &self.pwszHashAlgId).finish()
    }
}
unsafe impl ::windows::core::Abi for WEBAUTHN_CLIENT_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WEBAUTHN_CLIENT_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WEBAUTHN_CLIENT_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for WEBAUTHN_CLIENT_DATA {}
impl ::core::default::Default for WEBAUTHN_CLIENT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CLIENT_DATA_CURRENT_VERSION: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WEBAUTHN_COMMON_ATTESTATION {
    pub dwVersion: u32,
    pub pwszAlg: ::windows::core::PCWSTR,
    pub lAlg: i32,
    pub cbSignature: u32,
    pub pbSignature: *mut u8,
    pub cX5c: u32,
    pub pX5c: *mut WEBAUTHN_X5C,
    pub pwszVer: ::windows::core::PCWSTR,
    pub cbCertInfo: u32,
    pub pbCertInfo: *mut u8,
    pub cbPubArea: u32,
    pub pbPubArea: *mut u8,
}
impl ::core::marker::Copy for WEBAUTHN_COMMON_ATTESTATION {}
impl ::core::clone::Clone for WEBAUTHN_COMMON_ATTESTATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WEBAUTHN_COMMON_ATTESTATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_COMMON_ATTESTATION").field("dwVersion", &self.dwVersion).field("pwszAlg", &self.pwszAlg).field("lAlg", &self.lAlg).field("cbSignature", &self.cbSignature).field("pbSignature", &self.pbSignature).field("cX5c", &self.cX5c).field("pX5c", &self.pX5c).field("pwszVer", &self.pwszVer).field("cbCertInfo", &self.cbCertInfo).field("pbCertInfo", &self.pbCertInfo).field("cbPubArea", &self.cbPubArea).field("pbPubArea", &self.pbPubArea).finish()
    }
}
unsafe impl ::windows::core::Abi for WEBAUTHN_COMMON_ATTESTATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WEBAUTHN_COMMON_ATTESTATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WEBAUTHN_COMMON_ATTESTATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WEBAUTHN_COMMON_ATTESTATION {}
impl ::core::default::Default for WEBAUTHN_COMMON_ATTESTATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_COMMON_ATTESTATION_CURRENT_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_COSE_ALGORITHM_ECDSA_P256_WITH_SHA256: i32 = -7i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_COSE_ALGORITHM_ECDSA_P384_WITH_SHA384: i32 = -35i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_COSE_ALGORITHM_ECDSA_P521_WITH_SHA512: i32 = -36i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_COSE_ALGORITHM_RSASSA_PKCS1_V1_5_WITH_SHA256: i32 = -257i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_COSE_ALGORITHM_RSASSA_PKCS1_V1_5_WITH_SHA384: i32 = -258i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_COSE_ALGORITHM_RSASSA_PKCS1_V1_5_WITH_SHA512: i32 = -259i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_COSE_ALGORITHM_RSA_PSS_WITH_SHA256: i32 = -37i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_COSE_ALGORITHM_RSA_PSS_WITH_SHA384: i32 = -38i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_COSE_ALGORITHM_RSA_PSS_WITH_SHA512: i32 = -39i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WEBAUTHN_COSE_CREDENTIAL_PARAMETER {
    pub dwVersion: u32,
    pub pwszCredentialType: ::windows::core::PCWSTR,
    pub lAlg: i32,
}
impl ::core::marker::Copy for WEBAUTHN_COSE_CREDENTIAL_PARAMETER {}
impl ::core::clone::Clone for WEBAUTHN_COSE_CREDENTIAL_PARAMETER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WEBAUTHN_COSE_CREDENTIAL_PARAMETER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_COSE_CREDENTIAL_PARAMETER").field("dwVersion", &self.dwVersion).field("pwszCredentialType", &self.pwszCredentialType).field("lAlg", &self.lAlg).finish()
    }
}
unsafe impl ::windows::core::Abi for WEBAUTHN_COSE_CREDENTIAL_PARAMETER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WEBAUTHN_COSE_CREDENTIAL_PARAMETER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WEBAUTHN_COSE_CREDENTIAL_PARAMETER>()) == 0 }
    }
}
impl ::core::cmp::Eq for WEBAUTHN_COSE_CREDENTIAL_PARAMETER {}
impl ::core::default::Default for WEBAUTHN_COSE_CREDENTIAL_PARAMETER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WEBAUTHN_COSE_CREDENTIAL_PARAMETERS {
    pub cCredentialParameters: u32,
    pub pCredentialParameters: *mut WEBAUTHN_COSE_CREDENTIAL_PARAMETER,
}
impl ::core::marker::Copy for WEBAUTHN_COSE_CREDENTIAL_PARAMETERS {}
impl ::core::clone::Clone for WEBAUTHN_COSE_CREDENTIAL_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WEBAUTHN_COSE_CREDENTIAL_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_COSE_CREDENTIAL_PARAMETERS").field("cCredentialParameters", &self.cCredentialParameters).field("pCredentialParameters", &self.pCredentialParameters).finish()
    }
}
unsafe impl ::windows::core::Abi for WEBAUTHN_COSE_CREDENTIAL_PARAMETERS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WEBAUTHN_COSE_CREDENTIAL_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WEBAUTHN_COSE_CREDENTIAL_PARAMETERS>()) == 0 }
    }
}
impl ::core::cmp::Eq for WEBAUTHN_COSE_CREDENTIAL_PARAMETERS {}
impl ::core::default::Default for WEBAUTHN_COSE_CREDENTIAL_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_COSE_CREDENTIAL_PARAMETER_CURRENT_VERSION: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WEBAUTHN_CREDENTIAL {
    pub dwVersion: u32,
    pub cbId: u32,
    pub pbId: *mut u8,
    pub pwszCredentialType: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for WEBAUTHN_CREDENTIAL {}
impl ::core::clone::Clone for WEBAUTHN_CREDENTIAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WEBAUTHN_CREDENTIAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_CREDENTIAL").field("dwVersion", &self.dwVersion).field("cbId", &self.cbId).field("pbId", &self.pbId).field("pwszCredentialType", &self.pwszCredentialType).finish()
    }
}
unsafe impl ::windows::core::Abi for WEBAUTHN_CREDENTIAL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WEBAUTHN_CREDENTIAL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WEBAUTHN_CREDENTIAL>()) == 0 }
    }
}
impl ::core::cmp::Eq for WEBAUTHN_CREDENTIAL {}
impl ::core::default::Default for WEBAUTHN_CREDENTIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WEBAUTHN_CREDENTIALS {
    pub cCredentials: u32,
    pub pCredentials: *mut WEBAUTHN_CREDENTIAL,
}
impl ::core::marker::Copy for WEBAUTHN_CREDENTIALS {}
impl ::core::clone::Clone for WEBAUTHN_CREDENTIALS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WEBAUTHN_CREDENTIALS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_CREDENTIALS").field("cCredentials", &self.cCredentials).field("pCredentials", &self.pCredentials).finish()
    }
}
unsafe impl ::windows::core::Abi for WEBAUTHN_CREDENTIALS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WEBAUTHN_CREDENTIALS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WEBAUTHN_CREDENTIALS>()) == 0 }
    }
}
impl ::core::cmp::Eq for WEBAUTHN_CREDENTIALS {}
impl ::core::default::Default for WEBAUTHN_CREDENTIALS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WEBAUTHN_CREDENTIAL_ATTESTATION {
    pub dwVersion: u32,
    pub pwszFormatType: ::windows::core::PCWSTR,
    pub cbAuthenticatorData: u32,
    pub pbAuthenticatorData: *mut u8,
    pub cbAttestation: u32,
    pub pbAttestation: *mut u8,
    pub dwAttestationDecodeType: u32,
    pub pvAttestationDecode: *mut ::core::ffi::c_void,
    pub cbAttestationObject: u32,
    pub pbAttestationObject: *mut u8,
    pub cbCredentialId: u32,
    pub pbCredentialId: *mut u8,
    pub Extensions: WEBAUTHN_EXTENSIONS,
    pub dwUsedTransport: u32,
    pub bEpAtt: super::super::Foundation::BOOL,
    pub bLargeBlobSupported: super::super::Foundation::BOOL,
    pub bResidentKey: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WEBAUTHN_CREDENTIAL_ATTESTATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WEBAUTHN_CREDENTIAL_ATTESTATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WEBAUTHN_CREDENTIAL_ATTESTATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_CREDENTIAL_ATTESTATION")
            .field("dwVersion", &self.dwVersion)
            .field("pwszFormatType", &self.pwszFormatType)
            .field("cbAuthenticatorData", &self.cbAuthenticatorData)
            .field("pbAuthenticatorData", &self.pbAuthenticatorData)
            .field("cbAttestation", &self.cbAttestation)
            .field("pbAttestation", &self.pbAttestation)
            .field("dwAttestationDecodeType", &self.dwAttestationDecodeType)
            .field("pvAttestationDecode", &self.pvAttestationDecode)
            .field("cbAttestationObject", &self.cbAttestationObject)
            .field("pbAttestationObject", &self.pbAttestationObject)
            .field("cbCredentialId", &self.cbCredentialId)
            .field("pbCredentialId", &self.pbCredentialId)
            .field("Extensions", &self.Extensions)
            .field("dwUsedTransport", &self.dwUsedTransport)
            .field("bEpAtt", &self.bEpAtt)
            .field("bLargeBlobSupported", &self.bLargeBlobSupported)
            .field("bResidentKey", &self.bResidentKey)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WEBAUTHN_CREDENTIAL_ATTESTATION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WEBAUTHN_CREDENTIAL_ATTESTATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WEBAUTHN_CREDENTIAL_ATTESTATION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WEBAUTHN_CREDENTIAL_ATTESTATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WEBAUTHN_CREDENTIAL_ATTESTATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_CURRENT_VERSION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_3: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_4: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CREDENTIAL_CURRENT_VERSION: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WEBAUTHN_CREDENTIAL_EX {
    pub dwVersion: u32,
    pub cbId: u32,
    pub pbId: *mut u8,
    pub pwszCredentialType: ::windows::core::PCWSTR,
    pub dwTransports: u32,
}
impl ::core::marker::Copy for WEBAUTHN_CREDENTIAL_EX {}
impl ::core::clone::Clone for WEBAUTHN_CREDENTIAL_EX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WEBAUTHN_CREDENTIAL_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_CREDENTIAL_EX").field("dwVersion", &self.dwVersion).field("cbId", &self.cbId).field("pbId", &self.pbId).field("pwszCredentialType", &self.pwszCredentialType).field("dwTransports", &self.dwTransports).finish()
    }
}
unsafe impl ::windows::core::Abi for WEBAUTHN_CREDENTIAL_EX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WEBAUTHN_CREDENTIAL_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WEBAUTHN_CREDENTIAL_EX>()) == 0 }
    }
}
impl ::core::cmp::Eq for WEBAUTHN_CREDENTIAL_EX {}
impl ::core::default::Default for WEBAUTHN_CREDENTIAL_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CREDENTIAL_EX_CURRENT_VERSION: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WEBAUTHN_CREDENTIAL_LIST {
    pub cCredentials: u32,
    pub ppCredentials: *mut *mut WEBAUTHN_CREDENTIAL_EX,
}
impl ::core::marker::Copy for WEBAUTHN_CREDENTIAL_LIST {}
impl ::core::clone::Clone for WEBAUTHN_CREDENTIAL_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WEBAUTHN_CREDENTIAL_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_CREDENTIAL_LIST").field("cCredentials", &self.cCredentials).field("ppCredentials", &self.ppCredentials).finish()
    }
}
unsafe impl ::windows::core::Abi for WEBAUTHN_CREDENTIAL_LIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WEBAUTHN_CREDENTIAL_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WEBAUTHN_CREDENTIAL_LIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for WEBAUTHN_CREDENTIAL_LIST {}
impl ::core::default::Default for WEBAUTHN_CREDENTIAL_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CREDENTIAL_TYPE_PUBLIC_KEY: &str = "public-key";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WEBAUTHN_CRED_BLOB_EXTENSION {
    pub cbCredBlob: u32,
    pub pbCredBlob: *mut u8,
}
impl ::core::marker::Copy for WEBAUTHN_CRED_BLOB_EXTENSION {}
impl ::core::clone::Clone for WEBAUTHN_CRED_BLOB_EXTENSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WEBAUTHN_CRED_BLOB_EXTENSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_CRED_BLOB_EXTENSION").field("cbCredBlob", &self.cbCredBlob).field("pbCredBlob", &self.pbCredBlob).finish()
    }
}
unsafe impl ::windows::core::Abi for WEBAUTHN_CRED_BLOB_EXTENSION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WEBAUTHN_CRED_BLOB_EXTENSION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WEBAUTHN_CRED_BLOB_EXTENSION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WEBAUTHN_CRED_BLOB_EXTENSION {}
impl ::core::default::Default for WEBAUTHN_CRED_BLOB_EXTENSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CRED_LARGE_BLOB_OPERATION_DELETE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CRED_LARGE_BLOB_OPERATION_GET: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CRED_LARGE_BLOB_OPERATION_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CRED_LARGE_BLOB_OPERATION_SET: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_AUTHENTICATOR_ERROR: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_INVALID_DATA: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_INVALID_PARAMETER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_LACK_OF_SPACE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_MULTIPLE_CREDENTIALS: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_NOT_FOUND: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_NOT_SUPPORTED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_PLATFORM_ERROR: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_SUCCESS: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WEBAUTHN_CRED_PROTECT_EXTENSION_IN {
    pub dwCredProtect: u32,
    pub bRequireCredProtect: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WEBAUTHN_CRED_PROTECT_EXTENSION_IN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WEBAUTHN_CRED_PROTECT_EXTENSION_IN {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WEBAUTHN_CRED_PROTECT_EXTENSION_IN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_CRED_PROTECT_EXTENSION_IN").field("dwCredProtect", &self.dwCredProtect).field("bRequireCredProtect", &self.bRequireCredProtect).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WEBAUTHN_CRED_PROTECT_EXTENSION_IN {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WEBAUTHN_CRED_PROTECT_EXTENSION_IN {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WEBAUTHN_CRED_PROTECT_EXTENSION_IN>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WEBAUTHN_CRED_PROTECT_EXTENSION_IN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WEBAUTHN_CRED_PROTECT_EXTENSION_IN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CTAP_TRANSPORT_BLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CTAP_TRANSPORT_FLAGS_MASK: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CTAP_TRANSPORT_INTERNAL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CTAP_TRANSPORT_NFC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CTAP_TRANSPORT_TEST: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CTAP_TRANSPORT_USB: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_ENTERPRISE_ATTESTATION_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_ENTERPRISE_ATTESTATION_PLATFORM_MANAGED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_ENTERPRISE_ATTESTATION_VENDOR_FACILITATED: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WEBAUTHN_EXTENSION {
    pub pwszExtensionIdentifier: ::windows::core::PCWSTR,
    pub cbExtension: u32,
    pub pvExtension: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WEBAUTHN_EXTENSION {}
impl ::core::clone::Clone for WEBAUTHN_EXTENSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WEBAUTHN_EXTENSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_EXTENSION").field("pwszExtensionIdentifier", &self.pwszExtensionIdentifier).field("cbExtension", &self.cbExtension).field("pvExtension", &self.pvExtension).finish()
    }
}
unsafe impl ::windows::core::Abi for WEBAUTHN_EXTENSION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WEBAUTHN_EXTENSION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WEBAUTHN_EXTENSION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WEBAUTHN_EXTENSION {}
impl ::core::default::Default for WEBAUTHN_EXTENSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WEBAUTHN_EXTENSIONS {
    pub cExtensions: u32,
    pub pExtensions: *mut WEBAUTHN_EXTENSION,
}
impl ::core::marker::Copy for WEBAUTHN_EXTENSIONS {}
impl ::core::clone::Clone for WEBAUTHN_EXTENSIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WEBAUTHN_EXTENSIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_EXTENSIONS").field("cExtensions", &self.cExtensions).field("pExtensions", &self.pExtensions).finish()
    }
}
unsafe impl ::windows::core::Abi for WEBAUTHN_EXTENSIONS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WEBAUTHN_EXTENSIONS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WEBAUTHN_EXTENSIONS>()) == 0 }
    }
}
impl ::core::cmp::Eq for WEBAUTHN_EXTENSIONS {}
impl ::core::default::Default for WEBAUTHN_EXTENSIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_EXTENSIONS_IDENTIFIER_CRED_BLOB: &str = "credBlob";
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_EXTENSIONS_IDENTIFIER_CRED_PROTECT: &str = "credProtect";
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_EXTENSIONS_IDENTIFIER_HMAC_SECRET: &str = "hmac-secret";
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_EXTENSIONS_IDENTIFIER_MIN_PIN_LENGTH: &str = "minPinLength";
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_HASH_ALGORITHM_SHA_256: &str = "SHA-256";
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_HASH_ALGORITHM_SHA_384: &str = "SHA-384";
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_HASH_ALGORITHM_SHA_512: &str = "SHA-512";
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_LARGE_BLOB_SUPPORT_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_LARGE_BLOB_SUPPORT_PREFERRED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_LARGE_BLOB_SUPPORT_REQUIRED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_MAX_USER_ID_LENGTH: u32 = 64u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WEBAUTHN_RP_ENTITY_INFORMATION {
    pub dwVersion: u32,
    pub pwszId: ::windows::core::PCWSTR,
    pub pwszName: ::windows::core::PCWSTR,
    pub pwszIcon: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for WEBAUTHN_RP_ENTITY_INFORMATION {}
impl ::core::clone::Clone for WEBAUTHN_RP_ENTITY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WEBAUTHN_RP_ENTITY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_RP_ENTITY_INFORMATION").field("dwVersion", &self.dwVersion).field("pwszId", &self.pwszId).field("pwszName", &self.pwszName).field("pwszIcon", &self.pwszIcon).finish()
    }
}
unsafe impl ::windows::core::Abi for WEBAUTHN_RP_ENTITY_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WEBAUTHN_RP_ENTITY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WEBAUTHN_RP_ENTITY_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WEBAUTHN_RP_ENTITY_INFORMATION {}
impl ::core::default::Default for WEBAUTHN_RP_ENTITY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_RP_ENTITY_INFORMATION_CURRENT_VERSION: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WEBAUTHN_USER_ENTITY_INFORMATION {
    pub dwVersion: u32,
    pub cbId: u32,
    pub pbId: *mut u8,
    pub pwszName: ::windows::core::PCWSTR,
    pub pwszIcon: ::windows::core::PCWSTR,
    pub pwszDisplayName: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for WEBAUTHN_USER_ENTITY_INFORMATION {}
impl ::core::clone::Clone for WEBAUTHN_USER_ENTITY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WEBAUTHN_USER_ENTITY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_USER_ENTITY_INFORMATION").field("dwVersion", &self.dwVersion).field("cbId", &self.cbId).field("pbId", &self.pbId).field("pwszName", &self.pwszName).field("pwszIcon", &self.pwszIcon).field("pwszDisplayName", &self.pwszDisplayName).finish()
    }
}
unsafe impl ::windows::core::Abi for WEBAUTHN_USER_ENTITY_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WEBAUTHN_USER_ENTITY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WEBAUTHN_USER_ENTITY_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WEBAUTHN_USER_ENTITY_INFORMATION {}
impl ::core::default::Default for WEBAUTHN_USER_ENTITY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_USER_ENTITY_INFORMATION_CURRENT_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_USER_VERIFICATION_ANY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_USER_VERIFICATION_OPTIONAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_USER_VERIFICATION_OPTIONAL_WITH_CREDENTIAL_ID_LIST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_USER_VERIFICATION_REQUIRED: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_USER_VERIFICATION_REQUIREMENT_ANY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_USER_VERIFICATION_REQUIREMENT_DISCOURAGED: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_USER_VERIFICATION_REQUIREMENT_PREFERRED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_USER_VERIFICATION_REQUIREMENT_REQUIRED: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WEBAUTHN_X5C {
    pub cbData: u32,
    pub pbData: *mut u8,
}
impl ::core::marker::Copy for WEBAUTHN_X5C {}
impl ::core::clone::Clone for WEBAUTHN_X5C {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WEBAUTHN_X5C {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_X5C").field("cbData", &self.cbData).field("pbData", &self.pbData).finish()
    }
}
unsafe impl ::windows::core::Abi for WEBAUTHN_X5C {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WEBAUTHN_X5C {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WEBAUTHN_X5C>()) == 0 }
    }
}
impl ::core::cmp::Eq for WEBAUTHN_X5C {}
impl ::core::default::Default for WEBAUTHN_X5C {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_ABANDON_MESSAGE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(channelinstance: *const ::core::ffi::c_void, message: *const WS_MESSAGE, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_ABORT_CHANNEL_CALLBACK = ::core::option::Option<unsafe extern "system" fn(channelinstance: *const ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_ABORT_LISTENER_CALLBACK = ::core::option::Option<unsafe extern "system" fn(listenerinstance: *const ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_ACCEPT_CHANNEL_CALLBACK = ::core::option::Option<unsafe extern "system" fn(listenerinstance: *const ::core::ffi::c_void, channelinstance: *const ::core::ffi::c_void, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_ADDRESSING_VERSION(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ADDRESSING_VERSION_0_9: WS_ADDRESSING_VERSION = WS_ADDRESSING_VERSION(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ADDRESSING_VERSION_1_0: WS_ADDRESSING_VERSION = WS_ADDRESSING_VERSION(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ADDRESSING_VERSION_TRANSPORT: WS_ADDRESSING_VERSION = WS_ADDRESSING_VERSION(3i32);
impl ::core::marker::Copy for WS_ADDRESSING_VERSION {}
impl ::core::clone::Clone for WS_ADDRESSING_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_ADDRESSING_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_ADDRESSING_VERSION {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_ADDRESSING_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_ADDRESSING_VERSION").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_ANY_ATTRIBUTE {
    pub localName: WS_XML_STRING,
    pub ns: WS_XML_STRING,
    pub value: *mut WS_XML_TEXT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_ANY_ATTRIBUTE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_ANY_ATTRIBUTE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_ANY_ATTRIBUTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ANY_ATTRIBUTE").field("localName", &self.localName).field("ns", &self.ns).field("value", &self.value).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_ANY_ATTRIBUTE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_ANY_ATTRIBUTE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_ANY_ATTRIBUTE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_ANY_ATTRIBUTE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_ANY_ATTRIBUTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_ANY_ATTRIBUTES {
    pub attributes: *mut WS_ANY_ATTRIBUTE,
    pub attributeCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_ANY_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_ANY_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_ANY_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ANY_ATTRIBUTES").field("attributes", &self.attributes).field("attributeCount", &self.attributeCount).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_ANY_ATTRIBUTES {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_ANY_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_ANY_ATTRIBUTES>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_ANY_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_ANY_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_ASYNC_CALLBACK = ::core::option::Option<unsafe extern "system" fn(errorcode: ::windows::core::HRESULT, callbackmodel: WS_CALLBACK_MODEL, callbackstate: *const ::core::ffi::c_void)>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_ASYNC_CONTEXT {
    pub callback: WS_ASYNC_CALLBACK,
    pub callbackState: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WS_ASYNC_CONTEXT {}
impl ::core::clone::Clone for WS_ASYNC_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_ASYNC_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ASYNC_CONTEXT").field("callback", &self.callback.map(|f| f as usize)).field("callbackState", &self.callbackState).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_ASYNC_CONTEXT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_ASYNC_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_ASYNC_CONTEXT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_ASYNC_CONTEXT {}
impl ::core::default::Default for WS_ASYNC_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_ASYNC_FUNCTION = ::core::option::Option<unsafe extern "system" fn(hr: ::windows::core::HRESULT, callbackmodel: WS_CALLBACK_MODEL, callbackstate: *const ::core::ffi::c_void, next: *mut WS_ASYNC_OPERATION, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_ASYNC_OPERATION {
    pub function: WS_ASYNC_FUNCTION,
}
impl ::core::marker::Copy for WS_ASYNC_OPERATION {}
impl ::core::clone::Clone for WS_ASYNC_OPERATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_ASYNC_OPERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ASYNC_OPERATION").field("function", &self.function.map(|f| f as usize)).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_ASYNC_OPERATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_ASYNC_OPERATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_ASYNC_OPERATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_ASYNC_OPERATION {}
impl ::core::default::Default for WS_ASYNC_OPERATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_ASYNC_STATE {
    pub internal0: *mut ::core::ffi::c_void,
    pub internal1: *mut ::core::ffi::c_void,
    pub internal2: *mut ::core::ffi::c_void,
    pub internal3: *mut ::core::ffi::c_void,
    pub internal4: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WS_ASYNC_STATE {}
impl ::core::clone::Clone for WS_ASYNC_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_ASYNC_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ASYNC_STATE").field("internal0", &self.internal0).field("internal1", &self.internal1).field("internal2", &self.internal2).field("internal3", &self.internal3).field("internal4", &self.internal4).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_ASYNC_STATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_ASYNC_STATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_ASYNC_STATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_ASYNC_STATE {}
impl ::core::default::Default for WS_ASYNC_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_ATTRIBUTE_DESCRIPTION {
    pub attributeLocalName: *mut WS_XML_STRING,
    pub attributeNs: *mut WS_XML_STRING,
    pub r#type: WS_TYPE,
    pub typeDescription: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_ATTRIBUTE_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_ATTRIBUTE_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_ATTRIBUTE_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ATTRIBUTE_DESCRIPTION").field("attributeLocalName", &self.attributeLocalName).field("attributeNs", &self.attributeNs).field("type", &self.r#type).field("typeDescription", &self.typeDescription).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_ATTRIBUTE_DESCRIPTION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_ATTRIBUTE_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_ATTRIBUTE_DESCRIPTION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_ATTRIBUTE_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_ATTRIBUTE_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_BINDING_TEMPLATE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_BINDING_TEMPLATE_TYPE: WS_BINDING_TEMPLATE_TYPE = WS_BINDING_TEMPLATE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_SSL_BINDING_TEMPLATE_TYPE: WS_BINDING_TEMPLATE_TYPE = WS_BINDING_TEMPLATE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_HEADER_AUTH_BINDING_TEMPLATE_TYPE: WS_BINDING_TEMPLATE_TYPE = WS_BINDING_TEMPLATE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_SSL_HEADER_AUTH_BINDING_TEMPLATE_TYPE: WS_BINDING_TEMPLATE_TYPE = WS_BINDING_TEMPLATE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_SSL_USERNAME_BINDING_TEMPLATE_TYPE: WS_BINDING_TEMPLATE_TYPE = WS_BINDING_TEMPLATE_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_SSL_KERBEROS_APREQ_BINDING_TEMPLATE_TYPE: WS_BINDING_TEMPLATE_TYPE = WS_BINDING_TEMPLATE_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TCP_BINDING_TEMPLATE_TYPE: WS_BINDING_TEMPLATE_TYPE = WS_BINDING_TEMPLATE_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TCP_SSPI_BINDING_TEMPLATE_TYPE: WS_BINDING_TEMPLATE_TYPE = WS_BINDING_TEMPLATE_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TCP_SSPI_USERNAME_BINDING_TEMPLATE_TYPE: WS_BINDING_TEMPLATE_TYPE = WS_BINDING_TEMPLATE_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TCP_SSPI_KERBEROS_APREQ_BINDING_TEMPLATE_TYPE: WS_BINDING_TEMPLATE_TYPE = WS_BINDING_TEMPLATE_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE_TYPE: WS_BINDING_TEMPLATE_TYPE = WS_BINDING_TEMPLATE_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE_TYPE: WS_BINDING_TEMPLATE_TYPE = WS_BINDING_TEMPLATE_TYPE(11i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE_TYPE: WS_BINDING_TEMPLATE_TYPE = WS_BINDING_TEMPLATE_TYPE(12i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE_TYPE: WS_BINDING_TEMPLATE_TYPE = WS_BINDING_TEMPLATE_TYPE(13i32);
impl ::core::marker::Copy for WS_BINDING_TEMPLATE_TYPE {}
impl ::core::clone::Clone for WS_BINDING_TEMPLATE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_BINDING_TEMPLATE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_BINDING_TEMPLATE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_BINDING_TEMPLATE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_BINDING_TEMPLATE_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_BOOL_DESCRIPTION {
    pub value: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_BOOL_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_BOOL_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_BOOL_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_BOOL_DESCRIPTION").field("value", &self.value).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_BOOL_DESCRIPTION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_BOOL_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_BOOL_DESCRIPTION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_BOOL_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_BOOL_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_BUFFERS {
    pub bufferCount: u32,
    pub buffers: *mut WS_BYTES,
}
impl ::core::marker::Copy for WS_BUFFERS {}
impl ::core::clone::Clone for WS_BUFFERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_BUFFERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_BUFFERS").field("bufferCount", &self.bufferCount).field("buffers", &self.buffers).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_BUFFERS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_BUFFERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_BUFFERS>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_BUFFERS {}
impl ::core::default::Default for WS_BUFFERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_BYTES {
    pub length: u32,
    pub bytes: *mut u8,
}
impl ::core::marker::Copy for WS_BYTES {}
impl ::core::clone::Clone for WS_BYTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_BYTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_BYTES").field("length", &self.length).field("bytes", &self.bytes).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_BYTES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_BYTES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_BYTES>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_BYTES {}
impl ::core::default::Default for WS_BYTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_BYTES_DESCRIPTION {
    pub minByteCount: u32,
    pub maxByteCount: u32,
}
impl ::core::marker::Copy for WS_BYTES_DESCRIPTION {}
impl ::core::clone::Clone for WS_BYTES_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_BYTES_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_BYTES_DESCRIPTION").field("minByteCount", &self.minByteCount).field("maxByteCount", &self.maxByteCount).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_BYTES_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_BYTES_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_BYTES_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_BYTES_DESCRIPTION {}
impl ::core::default::Default for WS_BYTES_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_BYTE_ARRAY_DESCRIPTION {
    pub minByteCount: u32,
    pub maxByteCount: u32,
}
impl ::core::marker::Copy for WS_BYTE_ARRAY_DESCRIPTION {}
impl ::core::clone::Clone for WS_BYTE_ARRAY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_BYTE_ARRAY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_BYTE_ARRAY_DESCRIPTION").field("minByteCount", &self.minByteCount).field("maxByteCount", &self.maxByteCount).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_BYTE_ARRAY_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_BYTE_ARRAY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_BYTE_ARRAY_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_BYTE_ARRAY_DESCRIPTION {}
impl ::core::default::Default for WS_BYTE_ARRAY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_CALLBACK_MODEL(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SHORT_CALLBACK: WS_CALLBACK_MODEL = WS_CALLBACK_MODEL(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LONG_CALLBACK: WS_CALLBACK_MODEL = WS_CALLBACK_MODEL(1i32);
impl ::core::marker::Copy for WS_CALLBACK_MODEL {}
impl ::core::clone::Clone for WS_CALLBACK_MODEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_CALLBACK_MODEL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_CALLBACK_MODEL {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_CALLBACK_MODEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_CALLBACK_MODEL").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_CALL_PROPERTY {
    pub id: WS_CALL_PROPERTY_ID,
    pub value: *mut ::core::ffi::c_void,
    pub valueSize: u32,
}
impl ::core::marker::Copy for WS_CALL_PROPERTY {}
impl ::core::clone::Clone for WS_CALL_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_CALL_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CALL_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_CALL_PROPERTY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_CALL_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_CALL_PROPERTY>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_CALL_PROPERTY {}
impl ::core::default::Default for WS_CALL_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_CALL_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CALL_PROPERTY_CHECK_MUST_UNDERSTAND: WS_CALL_PROPERTY_ID = WS_CALL_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CALL_PROPERTY_SEND_MESSAGE_CONTEXT: WS_CALL_PROPERTY_ID = WS_CALL_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CALL_PROPERTY_RECEIVE_MESSAGE_CONTEXT: WS_CALL_PROPERTY_ID = WS_CALL_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CALL_PROPERTY_CALL_ID: WS_CALL_PROPERTY_ID = WS_CALL_PROPERTY_ID(3i32);
impl ::core::marker::Copy for WS_CALL_PROPERTY_ID {}
impl ::core::clone::Clone for WS_CALL_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_CALL_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_CALL_PROPERTY_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_CALL_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_CALL_PROPERTY_ID").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_CAPI_ASYMMETRIC_SECURITY_KEY_HANDLE {
    pub keyHandle: WS_SECURITY_KEY_HANDLE,
    pub provider: usize,
    pub keySpec: u32,
}
impl ::core::marker::Copy for WS_CAPI_ASYMMETRIC_SECURITY_KEY_HANDLE {}
impl ::core::clone::Clone for WS_CAPI_ASYMMETRIC_SECURITY_KEY_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_CAPI_ASYMMETRIC_SECURITY_KEY_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CAPI_ASYMMETRIC_SECURITY_KEY_HANDLE").field("keyHandle", &self.keyHandle).field("provider", &self.provider).field("keySpec", &self.keySpec).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_CAPI_ASYMMETRIC_SECURITY_KEY_HANDLE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_CAPI_ASYMMETRIC_SECURITY_KEY_HANDLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_CAPI_ASYMMETRIC_SECURITY_KEY_HANDLE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_CAPI_ASYMMETRIC_SECURITY_KEY_HANDLE {}
impl ::core::default::Default for WS_CAPI_ASYMMETRIC_SECURITY_KEY_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub type WS_CERTIFICATE_VALIDATION_CALLBACK = ::core::option::Option<unsafe extern "system" fn(certcontext: *const super::super::Security::Cryptography::CERT_CONTEXT, state: *const ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct WS_CERTIFICATE_VALIDATION_CALLBACK_CONTEXT {
    pub callback: WS_CERTIFICATE_VALIDATION_CALLBACK,
    pub state: *mut ::core::ffi::c_void,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for WS_CERTIFICATE_VALIDATION_CALLBACK_CONTEXT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for WS_CERTIFICATE_VALIDATION_CALLBACK_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for WS_CERTIFICATE_VALIDATION_CALLBACK_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CERTIFICATE_VALIDATION_CALLBACK_CONTEXT").field("callback", &self.callback.map(|f| f as usize)).field("state", &self.state).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::core::Abi for WS_CERTIFICATE_VALIDATION_CALLBACK_CONTEXT {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for WS_CERTIFICATE_VALIDATION_CALLBACK_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_CERTIFICATE_VALIDATION_CALLBACK_CONTEXT>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for WS_CERTIFICATE_VALIDATION_CALLBACK_CONTEXT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for WS_CERTIFICATE_VALIDATION_CALLBACK_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_CERT_CREDENTIAL {
    pub credentialType: WS_CERT_CREDENTIAL_TYPE,
}
impl ::core::marker::Copy for WS_CERT_CREDENTIAL {}
impl ::core::clone::Clone for WS_CERT_CREDENTIAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_CERT_CREDENTIAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CERT_CREDENTIAL").field("credentialType", &self.credentialType).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_CERT_CREDENTIAL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_CERT_CREDENTIAL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_CERT_CREDENTIAL>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_CERT_CREDENTIAL {}
impl ::core::default::Default for WS_CERT_CREDENTIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_CERT_CREDENTIAL_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SUBJECT_NAME_CERT_CREDENTIAL_TYPE: WS_CERT_CREDENTIAL_TYPE = WS_CERT_CREDENTIAL_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_THUMBPRINT_CERT_CREDENTIAL_TYPE: WS_CERT_CREDENTIAL_TYPE = WS_CERT_CREDENTIAL_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CUSTOM_CERT_CREDENTIAL_TYPE: WS_CERT_CREDENTIAL_TYPE = WS_CERT_CREDENTIAL_TYPE(3i32);
impl ::core::marker::Copy for WS_CERT_CREDENTIAL_TYPE {}
impl ::core::clone::Clone for WS_CERT_CREDENTIAL_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_CERT_CREDENTIAL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_CERT_CREDENTIAL_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_CERT_CREDENTIAL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_CERT_CREDENTIAL_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_CERT_ENDPOINT_IDENTITY {
    pub identity: WS_ENDPOINT_IDENTITY,
    pub rawCertificateData: WS_BYTES,
}
impl ::core::marker::Copy for WS_CERT_ENDPOINT_IDENTITY {}
impl ::core::clone::Clone for WS_CERT_ENDPOINT_IDENTITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_CERT_ENDPOINT_IDENTITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CERT_ENDPOINT_IDENTITY").field("identity", &self.identity).field("rawCertificateData", &self.rawCertificateData).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_CERT_ENDPOINT_IDENTITY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_CERT_ENDPOINT_IDENTITY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_CERT_ENDPOINT_IDENTITY>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_CERT_ENDPOINT_IDENTITY {}
impl ::core::default::Default for WS_CERT_ENDPOINT_IDENTITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CERT_FAILURE_CN_MISMATCH: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CERT_FAILURE_INVALID_DATE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CERT_FAILURE_REVOCATION_OFFLINE: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CERT_FAILURE_UNTRUSTED_ROOT: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CERT_FAILURE_WRONG_USAGE: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
pub type WS_CERT_ISSUER_LIST_NOTIFICATION_CALLBACK = ::core::option::Option<unsafe extern "system" fn(certissuerlistnotificationcallbackstate: *const ::core::ffi::c_void, issuerlist: *const super::super::Security::Authentication::Identity::SecPkgContext_IssuerListInfoEx, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_CERT_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    pub bindingConstraint: WS_SECURITY_BINDING_CONSTRAINT,
    pub bindingUsage: WS_MESSAGE_SECURITY_USAGE,
}
impl ::core::marker::Copy for WS_CERT_MESSAGE_SECURITY_BINDING_CONSTRAINT {}
impl ::core::clone::Clone for WS_CERT_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_CERT_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CERT_MESSAGE_SECURITY_BINDING_CONSTRAINT").field("bindingConstraint", &self.bindingConstraint).field("bindingUsage", &self.bindingUsage).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_CERT_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_CERT_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_CERT_MESSAGE_SECURITY_BINDING_CONSTRAINT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_CERT_MESSAGE_SECURITY_BINDING_CONSTRAINT {}
impl ::core::default::Default for WS_CERT_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct WS_CERT_SIGNED_SAML_AUTHENTICATOR {
    pub authenticator: WS_SAML_AUTHENTICATOR,
    pub trustedIssuerCerts: *const *const super::super::Security::Cryptography::CERT_CONTEXT,
    pub trustedIssuerCertCount: u32,
    pub decryptionCert: *const super::super::Security::Cryptography::CERT_CONTEXT,
    pub samlValidator: WS_VALIDATE_SAML_CALLBACK,
    pub samlValidatorCallbackState: *mut ::core::ffi::c_void,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for WS_CERT_SIGNED_SAML_AUTHENTICATOR {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for WS_CERT_SIGNED_SAML_AUTHENTICATOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for WS_CERT_SIGNED_SAML_AUTHENTICATOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CERT_SIGNED_SAML_AUTHENTICATOR").field("authenticator", &self.authenticator).field("trustedIssuerCerts", &self.trustedIssuerCerts).field("trustedIssuerCertCount", &self.trustedIssuerCertCount).field("decryptionCert", &self.decryptionCert).field("samlValidator", &self.samlValidator.map(|f| f as usize)).field("samlValidatorCallbackState", &self.samlValidatorCallbackState).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::core::Abi for WS_CERT_SIGNED_SAML_AUTHENTICATOR {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for WS_CERT_SIGNED_SAML_AUTHENTICATOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_CERT_SIGNED_SAML_AUTHENTICATOR>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for WS_CERT_SIGNED_SAML_AUTHENTICATOR {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for WS_CERT_SIGNED_SAML_AUTHENTICATOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WS_CHANNEL(pub u8);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_CHANNEL_BINDING(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_CHANNEL_BINDING: WS_CHANNEL_BINDING = WS_CHANNEL_BINDING(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TCP_CHANNEL_BINDING: WS_CHANNEL_BINDING = WS_CHANNEL_BINDING(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_UDP_CHANNEL_BINDING: WS_CHANNEL_BINDING = WS_CHANNEL_BINDING(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CUSTOM_CHANNEL_BINDING: WS_CHANNEL_BINDING = WS_CHANNEL_BINDING(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_NAMEDPIPE_CHANNEL_BINDING: WS_CHANNEL_BINDING = WS_CHANNEL_BINDING(4i32);
impl ::core::marker::Copy for WS_CHANNEL_BINDING {}
impl ::core::clone::Clone for WS_CHANNEL_BINDING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_CHANNEL_BINDING {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_CHANNEL_BINDING {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_CHANNEL_BINDING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_CHANNEL_BINDING").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_CHANNEL_DECODER {
    pub createContext: *mut ::core::ffi::c_void,
    pub createDecoderCallback: WS_CREATE_DECODER_CALLBACK,
    pub decoderGetContentTypeCallback: WS_DECODER_GET_CONTENT_TYPE_CALLBACK,
    pub decoderStartCallback: WS_DECODER_START_CALLBACK,
    pub decoderDecodeCallback: WS_DECODER_DECODE_CALLBACK,
    pub decoderEndCallback: WS_DECODER_END_CALLBACK,
    pub freeDecoderCallback: WS_FREE_DECODER_CALLBACK,
}
impl ::core::marker::Copy for WS_CHANNEL_DECODER {}
impl ::core::clone::Clone for WS_CHANNEL_DECODER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_CHANNEL_DECODER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CHANNEL_DECODER")
            .field("createContext", &self.createContext)
            .field("createDecoderCallback", &self.createDecoderCallback.map(|f| f as usize))
            .field("decoderGetContentTypeCallback", &self.decoderGetContentTypeCallback.map(|f| f as usize))
            .field("decoderStartCallback", &self.decoderStartCallback.map(|f| f as usize))
            .field("decoderDecodeCallback", &self.decoderDecodeCallback.map(|f| f as usize))
            .field("decoderEndCallback", &self.decoderEndCallback.map(|f| f as usize))
            .field("freeDecoderCallback", &self.freeDecoderCallback.map(|f| f as usize))
            .finish()
    }
}
unsafe impl ::windows::core::Abi for WS_CHANNEL_DECODER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_CHANNEL_DECODER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_CHANNEL_DECODER>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_CHANNEL_DECODER {}
impl ::core::default::Default for WS_CHANNEL_DECODER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_CHANNEL_ENCODER {
    pub createContext: *mut ::core::ffi::c_void,
    pub createEncoderCallback: WS_CREATE_ENCODER_CALLBACK,
    pub encoderGetContentTypeCallback: WS_ENCODER_GET_CONTENT_TYPE_CALLBACK,
    pub encoderStartCallback: WS_ENCODER_START_CALLBACK,
    pub encoderEncodeCallback: WS_ENCODER_ENCODE_CALLBACK,
    pub encoderEndCallback: WS_ENCODER_END_CALLBACK,
    pub freeEncoderCallback: WS_FREE_ENCODER_CALLBACK,
}
impl ::core::marker::Copy for WS_CHANNEL_ENCODER {}
impl ::core::clone::Clone for WS_CHANNEL_ENCODER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_CHANNEL_ENCODER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CHANNEL_ENCODER")
            .field("createContext", &self.createContext)
            .field("createEncoderCallback", &self.createEncoderCallback.map(|f| f as usize))
            .field("encoderGetContentTypeCallback", &self.encoderGetContentTypeCallback.map(|f| f as usize))
            .field("encoderStartCallback", &self.encoderStartCallback.map(|f| f as usize))
            .field("encoderEncodeCallback", &self.encoderEncodeCallback.map(|f| f as usize))
            .field("encoderEndCallback", &self.encoderEndCallback.map(|f| f as usize))
            .field("freeEncoderCallback", &self.freeEncoderCallback.map(|f| f as usize))
            .finish()
    }
}
unsafe impl ::windows::core::Abi for WS_CHANNEL_ENCODER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_CHANNEL_ENCODER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_CHANNEL_ENCODER>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_CHANNEL_ENCODER {}
impl ::core::default::Default for WS_CHANNEL_ENCODER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_CHANNEL_PROPERTIES {
    pub properties: *mut WS_CHANNEL_PROPERTY,
    pub propertyCount: u32,
}
impl ::core::marker::Copy for WS_CHANNEL_PROPERTIES {}
impl ::core::clone::Clone for WS_CHANNEL_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_CHANNEL_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CHANNEL_PROPERTIES").field("properties", &self.properties).field("propertyCount", &self.propertyCount).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_CHANNEL_PROPERTIES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_CHANNEL_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_CHANNEL_PROPERTIES>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_CHANNEL_PROPERTIES {}
impl ::core::default::Default for WS_CHANNEL_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_CHANNEL_PROPERTY {
    pub id: WS_CHANNEL_PROPERTY_ID,
    pub value: *mut ::core::ffi::c_void,
    pub valueSize: u32,
}
impl ::core::marker::Copy for WS_CHANNEL_PROPERTY {}
impl ::core::clone::Clone for WS_CHANNEL_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_CHANNEL_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CHANNEL_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_CHANNEL_PROPERTY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_CHANNEL_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_CHANNEL_PROPERTY>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_CHANNEL_PROPERTY {}
impl ::core::default::Default for WS_CHANNEL_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_CHANNEL_PROPERTY_CONSTRAINT {
    pub id: WS_CHANNEL_PROPERTY_ID,
    pub allowedValues: *mut ::core::ffi::c_void,
    pub allowedValuesSize: u32,
    pub out: WS_CHANNEL_PROPERTY_CONSTRAINT_0,
}
impl ::core::marker::Copy for WS_CHANNEL_PROPERTY_CONSTRAINT {}
impl ::core::clone::Clone for WS_CHANNEL_PROPERTY_CONSTRAINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_CHANNEL_PROPERTY_CONSTRAINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CHANNEL_PROPERTY_CONSTRAINT").field("id", &self.id).field("allowedValues", &self.allowedValues).field("allowedValuesSize", &self.allowedValuesSize).field("out", &self.out).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_CHANNEL_PROPERTY_CONSTRAINT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_CHANNEL_PROPERTY_CONSTRAINT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_CHANNEL_PROPERTY_CONSTRAINT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_CHANNEL_PROPERTY_CONSTRAINT {}
impl ::core::default::Default for WS_CHANNEL_PROPERTY_CONSTRAINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_CHANNEL_PROPERTY_CONSTRAINT_0 {
    pub channelProperty: WS_CHANNEL_PROPERTY,
}
impl ::core::marker::Copy for WS_CHANNEL_PROPERTY_CONSTRAINT_0 {}
impl ::core::clone::Clone for WS_CHANNEL_PROPERTY_CONSTRAINT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_CHANNEL_PROPERTY_CONSTRAINT_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CHANNEL_PROPERTY_CONSTRAINT_0").field("channelProperty", &self.channelProperty).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_CHANNEL_PROPERTY_CONSTRAINT_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_CHANNEL_PROPERTY_CONSTRAINT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_CHANNEL_PROPERTY_CONSTRAINT_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_CHANNEL_PROPERTY_CONSTRAINT_0 {}
impl ::core::default::Default for WS_CHANNEL_PROPERTY_CONSTRAINT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_CHANNEL_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_MAX_BUFFERED_MESSAGE_SIZE: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_MAX_STREAMED_MESSAGE_SIZE: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_MAX_STREAMED_START_SIZE: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_MAX_STREAMED_FLUSH_SIZE: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_ENCODING: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_ENVELOPE_VERSION: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_ADDRESSING_VERSION: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_MAX_SESSION_DICTIONARY_SIZE: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_STATE: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(8i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_ASYNC_CALLBACK_MODEL: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(9i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_IP_VERSION: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(10i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_RESOLVE_TIMEOUT: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(11i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_CONNECT_TIMEOUT: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(12i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_SEND_TIMEOUT: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(13i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_RECEIVE_RESPONSE_TIMEOUT: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(14i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_RECEIVE_TIMEOUT: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(15i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_CLOSE_TIMEOUT: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(16i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_ENABLE_TIMEOUTS: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(17i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_TRANSFER_MODE: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(18i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_MULTICAST_INTERFACE: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(19i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_MULTICAST_HOPS: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(20i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_REMOTE_ADDRESS: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(21i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_REMOTE_IP_ADDRESS: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(22i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_HTTP_CONNECTION_ID: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(23i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_CUSTOM_CHANNEL_CALLBACKS: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(24i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_CUSTOM_CHANNEL_PARAMETERS: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(25i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_CUSTOM_CHANNEL_INSTANCE: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(26i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_TRANSPORT_URL: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(27i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_NO_DELAY: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(28i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_SEND_KEEP_ALIVES: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(29i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_KEEP_ALIVE_TIME: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(30i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_KEEP_ALIVE_INTERVAL: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(31i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_MAX_HTTP_SERVER_CONNECTIONS: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(32i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_IS_SESSION_SHUT_DOWN: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(33i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_CHANNEL_TYPE: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(34i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_TRIM_BUFFERED_MESSAGE_SIZE: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(35i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_ENCODER: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(36i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_DECODER: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(37i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_PROTECTION_LEVEL: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(38i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_COOKIE_MODE: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(39i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_HTTP_PROXY_SETTING_MODE: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(40i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_CUSTOM_HTTP_PROXY: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(41i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_HTTP_MESSAGE_MAPPING: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(42i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_ENABLE_HTTP_REDIRECT: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(43i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_HTTP_REDIRECT_CALLBACK_CONTEXT: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(44i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_FAULTS_AS_ERRORS: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(45i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_ALLOW_UNSECURED_FAULTS: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(46i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_HTTP_SERVER_SPN: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(47i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_HTTP_PROXY_SPN: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(48i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_MAX_HTTP_REQUEST_HEADERS_BUFFER_SIZE: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(49i32);
impl ::core::marker::Copy for WS_CHANNEL_PROPERTY_ID {}
impl ::core::clone::Clone for WS_CHANNEL_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_CHANNEL_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_CHANNEL_PROPERTY_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_CHANNEL_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_CHANNEL_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_CHANNEL_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_STATE_CREATED: WS_CHANNEL_STATE = WS_CHANNEL_STATE(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_STATE_OPENING: WS_CHANNEL_STATE = WS_CHANNEL_STATE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_STATE_ACCEPTING: WS_CHANNEL_STATE = WS_CHANNEL_STATE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_STATE_OPEN: WS_CHANNEL_STATE = WS_CHANNEL_STATE(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_STATE_FAULTED: WS_CHANNEL_STATE = WS_CHANNEL_STATE(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_STATE_CLOSING: WS_CHANNEL_STATE = WS_CHANNEL_STATE(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_STATE_CLOSED: WS_CHANNEL_STATE = WS_CHANNEL_STATE(6i32);
impl ::core::marker::Copy for WS_CHANNEL_STATE {}
impl ::core::clone::Clone for WS_CHANNEL_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_CHANNEL_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_CHANNEL_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_CHANNEL_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_CHANNEL_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_CHANNEL_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_TYPE_INPUT: WS_CHANNEL_TYPE = WS_CHANNEL_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_TYPE_OUTPUT: WS_CHANNEL_TYPE = WS_CHANNEL_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_TYPE_SESSION: WS_CHANNEL_TYPE = WS_CHANNEL_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_TYPE_INPUT_SESSION: WS_CHANNEL_TYPE = WS_CHANNEL_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_TYPE_OUTPUT_SESSION: WS_CHANNEL_TYPE = WS_CHANNEL_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_TYPE_DUPLEX: WS_CHANNEL_TYPE = WS_CHANNEL_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_TYPE_DUPLEX_SESSION: WS_CHANNEL_TYPE = WS_CHANNEL_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_TYPE_REQUEST: WS_CHANNEL_TYPE = WS_CHANNEL_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_TYPE_REPLY: WS_CHANNEL_TYPE = WS_CHANNEL_TYPE(16i32);
impl ::core::marker::Copy for WS_CHANNEL_TYPE {}
impl ::core::clone::Clone for WS_CHANNEL_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_CHANNEL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_CHANNEL_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_CHANNEL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_CHANNEL_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_CHARSET(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHARSET_AUTO: WS_CHARSET = WS_CHARSET(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHARSET_UTF8: WS_CHARSET = WS_CHARSET(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHARSET_UTF16LE: WS_CHARSET = WS_CHARSET(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHARSET_UTF16BE: WS_CHARSET = WS_CHARSET(3i32);
impl ::core::marker::Copy for WS_CHARSET {}
impl ::core::clone::Clone for WS_CHARSET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_CHARSET {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_CHARSET {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_CHARSET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_CHARSET").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_CHAR_ARRAY_DESCRIPTION {
    pub minCharCount: u32,
    pub maxCharCount: u32,
}
impl ::core::marker::Copy for WS_CHAR_ARRAY_DESCRIPTION {}
impl ::core::clone::Clone for WS_CHAR_ARRAY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_CHAR_ARRAY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CHAR_ARRAY_DESCRIPTION").field("minCharCount", &self.minCharCount).field("maxCharCount", &self.maxCharCount).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_CHAR_ARRAY_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_CHAR_ARRAY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_CHAR_ARRAY_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_CHAR_ARRAY_DESCRIPTION {}
impl ::core::default::Default for WS_CHAR_ARRAY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_CLOSE_CHANNEL_CALLBACK = ::core::option::Option<unsafe extern "system" fn(channelinstance: *const ::core::ffi::c_void, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_CLOSE_LISTENER_CALLBACK = ::core::option::Option<unsafe extern "system" fn(listenerinstance: *const ::core::ffi::c_void, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_CONTRACT_DESCRIPTION {
    pub operationCount: u32,
    pub operations: *mut *mut WS_OPERATION_DESCRIPTION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_CONTRACT_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_CONTRACT_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_CONTRACT_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CONTRACT_DESCRIPTION").field("operationCount", &self.operationCount).field("operations", &self.operations).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_CONTRACT_DESCRIPTION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_CONTRACT_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_CONTRACT_DESCRIPTION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_CONTRACT_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_CONTRACT_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_COOKIE_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MANUAL_COOKIE_MODE: WS_COOKIE_MODE = WS_COOKIE_MODE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_AUTO_COOKIE_MODE: WS_COOKIE_MODE = WS_COOKIE_MODE(2i32);
impl ::core::marker::Copy for WS_COOKIE_MODE {}
impl ::core::clone::Clone for WS_COOKIE_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_COOKIE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_COOKIE_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_COOKIE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_COOKIE_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_CREATE_CHANNEL_CALLBACK = ::core::option::Option<unsafe extern "system" fn(channeltype: WS_CHANNEL_TYPE, channelparameters: *const ::core::ffi::c_void, channelparameterssize: u32, channelinstance: *mut *mut ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_CREATE_CHANNEL_FOR_LISTENER_CALLBACK = ::core::option::Option<unsafe extern "system" fn(listenerinstance: *const ::core::ffi::c_void, channelparameters: *const ::core::ffi::c_void, channelparameterssize: u32, channelinstance: *mut *mut ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_CREATE_DECODER_CALLBACK = ::core::option::Option<unsafe extern "system" fn(createcontext: *const ::core::ffi::c_void, readcallback: WS_READ_CALLBACK, readcontext: *const ::core::ffi::c_void, decodercontext: *mut *mut ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_CREATE_ENCODER_CALLBACK = ::core::option::Option<unsafe extern "system" fn(createcontext: *const ::core::ffi::c_void, writecallback: WS_WRITE_CALLBACK, writecontext: *const ::core::ffi::c_void, encodercontext: *mut *mut ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_CREATE_LISTENER_CALLBACK = ::core::option::Option<unsafe extern "system" fn(channeltype: WS_CHANNEL_TYPE, listenerparameters: *const ::core::ffi::c_void, listenerparameterssize: u32, listenerinstance: *mut *mut ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
pub struct WS_CUSTOM_CERT_CREDENTIAL {
    pub credential: WS_CERT_CREDENTIAL,
    pub getCertCallback: WS_GET_CERT_CALLBACK,
    pub getCertCallbackState: *mut ::core::ffi::c_void,
    pub certIssuerListNotificationCallback: WS_CERT_ISSUER_LIST_NOTIFICATION_CALLBACK,
    pub certIssuerListNotificationCallbackState: *mut ::core::ffi::c_void,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for WS_CUSTOM_CERT_CREDENTIAL {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for WS_CUSTOM_CERT_CREDENTIAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for WS_CUSTOM_CERT_CREDENTIAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CUSTOM_CERT_CREDENTIAL").field("credential", &self.credential).field("getCertCallback", &self.getCertCallback.map(|f| f as usize)).field("getCertCallbackState", &self.getCertCallbackState).field("certIssuerListNotificationCallback", &self.certIssuerListNotificationCallback.map(|f| f as usize)).field("certIssuerListNotificationCallbackState", &self.certIssuerListNotificationCallbackState).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::core::Abi for WS_CUSTOM_CERT_CREDENTIAL {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for WS_CUSTOM_CERT_CREDENTIAL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_CUSTOM_CERT_CREDENTIAL>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for WS_CUSTOM_CERT_CREDENTIAL {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for WS_CUSTOM_CERT_CREDENTIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_CUSTOM_CHANNEL_CALLBACKS {
    pub createChannelCallback: WS_CREATE_CHANNEL_CALLBACK,
    pub freeChannelCallback: WS_FREE_CHANNEL_CALLBACK,
    pub resetChannelCallback: WS_RESET_CHANNEL_CALLBACK,
    pub openChannelCallback: WS_OPEN_CHANNEL_CALLBACK,
    pub closeChannelCallback: WS_CLOSE_CHANNEL_CALLBACK,
    pub abortChannelCallback: WS_ABORT_CHANNEL_CALLBACK,
    pub getChannelPropertyCallback: WS_GET_CHANNEL_PROPERTY_CALLBACK,
    pub setChannelPropertyCallback: WS_SET_CHANNEL_PROPERTY_CALLBACK,
    pub writeMessageStartCallback: WS_WRITE_MESSAGE_START_CALLBACK,
    pub writeMessageEndCallback: WS_WRITE_MESSAGE_END_CALLBACK,
    pub readMessageStartCallback: WS_READ_MESSAGE_START_CALLBACK,
    pub readMessageEndCallback: WS_READ_MESSAGE_END_CALLBACK,
    pub abandonMessageCallback: WS_ABANDON_MESSAGE_CALLBACK,
    pub shutdownSessionChannelCallback: WS_SHUTDOWN_SESSION_CHANNEL_CALLBACK,
}
impl ::core::marker::Copy for WS_CUSTOM_CHANNEL_CALLBACKS {}
impl ::core::clone::Clone for WS_CUSTOM_CHANNEL_CALLBACKS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_CUSTOM_CHANNEL_CALLBACKS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CUSTOM_CHANNEL_CALLBACKS")
            .field("createChannelCallback", &self.createChannelCallback.map(|f| f as usize))
            .field("freeChannelCallback", &self.freeChannelCallback.map(|f| f as usize))
            .field("resetChannelCallback", &self.resetChannelCallback.map(|f| f as usize))
            .field("openChannelCallback", &self.openChannelCallback.map(|f| f as usize))
            .field("closeChannelCallback", &self.closeChannelCallback.map(|f| f as usize))
            .field("abortChannelCallback", &self.abortChannelCallback.map(|f| f as usize))
            .field("getChannelPropertyCallback", &self.getChannelPropertyCallback.map(|f| f as usize))
            .field("setChannelPropertyCallback", &self.setChannelPropertyCallback.map(|f| f as usize))
            .field("writeMessageStartCallback", &self.writeMessageStartCallback.map(|f| f as usize))
            .field("writeMessageEndCallback", &self.writeMessageEndCallback.map(|f| f as usize))
            .field("readMessageStartCallback", &self.readMessageStartCallback.map(|f| f as usize))
            .field("readMessageEndCallback", &self.readMessageEndCallback.map(|f| f as usize))
            .field("abandonMessageCallback", &self.abandonMessageCallback.map(|f| f as usize))
            .field("shutdownSessionChannelCallback", &self.shutdownSessionChannelCallback.map(|f| f as usize))
            .finish()
    }
}
unsafe impl ::windows::core::Abi for WS_CUSTOM_CHANNEL_CALLBACKS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_CUSTOM_CHANNEL_CALLBACKS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_CUSTOM_CHANNEL_CALLBACKS>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_CUSTOM_CHANNEL_CALLBACKS {}
impl ::core::default::Default for WS_CUSTOM_CHANNEL_CALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_CUSTOM_HTTP_PROXY {
    pub servers: WS_STRING,
    pub bypass: WS_STRING,
}
impl ::core::marker::Copy for WS_CUSTOM_HTTP_PROXY {}
impl ::core::clone::Clone for WS_CUSTOM_HTTP_PROXY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_CUSTOM_HTTP_PROXY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CUSTOM_HTTP_PROXY").field("servers", &self.servers).field("bypass", &self.bypass).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_CUSTOM_HTTP_PROXY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_CUSTOM_HTTP_PROXY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_CUSTOM_HTTP_PROXY>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_CUSTOM_HTTP_PROXY {}
impl ::core::default::Default for WS_CUSTOM_HTTP_PROXY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_CUSTOM_LISTENER_CALLBACKS {
    pub createListenerCallback: WS_CREATE_LISTENER_CALLBACK,
    pub freeListenerCallback: WS_FREE_LISTENER_CALLBACK,
    pub resetListenerCallback: WS_RESET_LISTENER_CALLBACK,
    pub openListenerCallback: WS_OPEN_LISTENER_CALLBACK,
    pub closeListenerCallback: WS_CLOSE_LISTENER_CALLBACK,
    pub abortListenerCallback: WS_ABORT_LISTENER_CALLBACK,
    pub getListenerPropertyCallback: WS_GET_LISTENER_PROPERTY_CALLBACK,
    pub setListenerPropertyCallback: WS_SET_LISTENER_PROPERTY_CALLBACK,
    pub createChannelForListenerCallback: WS_CREATE_CHANNEL_FOR_LISTENER_CALLBACK,
    pub acceptChannelCallback: WS_ACCEPT_CHANNEL_CALLBACK,
}
impl ::core::marker::Copy for WS_CUSTOM_LISTENER_CALLBACKS {}
impl ::core::clone::Clone for WS_CUSTOM_LISTENER_CALLBACKS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_CUSTOM_LISTENER_CALLBACKS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CUSTOM_LISTENER_CALLBACKS")
            .field("createListenerCallback", &self.createListenerCallback.map(|f| f as usize))
            .field("freeListenerCallback", &self.freeListenerCallback.map(|f| f as usize))
            .field("resetListenerCallback", &self.resetListenerCallback.map(|f| f as usize))
            .field("openListenerCallback", &self.openListenerCallback.map(|f| f as usize))
            .field("closeListenerCallback", &self.closeListenerCallback.map(|f| f as usize))
            .field("abortListenerCallback", &self.abortListenerCallback.map(|f| f as usize))
            .field("getListenerPropertyCallback", &self.getListenerPropertyCallback.map(|f| f as usize))
            .field("setListenerPropertyCallback", &self.setListenerPropertyCallback.map(|f| f as usize))
            .field("createChannelForListenerCallback", &self.createChannelForListenerCallback.map(|f| f as usize))
            .field("acceptChannelCallback", &self.acceptChannelCallback.map(|f| f as usize))
            .finish()
    }
}
unsafe impl ::windows::core::Abi for WS_CUSTOM_LISTENER_CALLBACKS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_CUSTOM_LISTENER_CALLBACKS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_CUSTOM_LISTENER_CALLBACKS>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_CUSTOM_LISTENER_CALLBACKS {}
impl ::core::default::Default for WS_CUSTOM_LISTENER_CALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_CUSTOM_TYPE_DESCRIPTION {
    pub size: u32,
    pub alignment: u32,
    pub readCallback: WS_READ_TYPE_CALLBACK,
    pub writeCallback: WS_WRITE_TYPE_CALLBACK,
    pub descriptionData: *mut ::core::ffi::c_void,
    pub isDefaultValueCallback: WS_IS_DEFAULT_VALUE_CALLBACK,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_CUSTOM_TYPE_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_CUSTOM_TYPE_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_CUSTOM_TYPE_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CUSTOM_TYPE_DESCRIPTION").field("size", &self.size).field("alignment", &self.alignment).field("readCallback", &self.readCallback.map(|f| f as usize)).field("writeCallback", &self.writeCallback.map(|f| f as usize)).field("descriptionData", &self.descriptionData).field("isDefaultValueCallback", &self.isDefaultValueCallback.map(|f| f as usize)).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_CUSTOM_TYPE_DESCRIPTION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_CUSTOM_TYPE_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_CUSTOM_TYPE_DESCRIPTION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_CUSTOM_TYPE_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_CUSTOM_TYPE_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_DATETIME {
    pub ticks: u64,
    pub format: WS_DATETIME_FORMAT,
}
impl ::core::marker::Copy for WS_DATETIME {}
impl ::core::clone::Clone for WS_DATETIME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_DATETIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_DATETIME").field("ticks", &self.ticks).field("format", &self.format).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_DATETIME {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_DATETIME {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_DATETIME>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_DATETIME {}
impl ::core::default::Default for WS_DATETIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_DATETIME_DESCRIPTION {
    pub minValue: WS_DATETIME,
    pub maxValue: WS_DATETIME,
}
impl ::core::marker::Copy for WS_DATETIME_DESCRIPTION {}
impl ::core::clone::Clone for WS_DATETIME_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_DATETIME_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_DATETIME_DESCRIPTION").field("minValue", &self.minValue).field("maxValue", &self.maxValue).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_DATETIME_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_DATETIME_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_DATETIME_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_DATETIME_DESCRIPTION {}
impl ::core::default::Default for WS_DATETIME_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_DATETIME_FORMAT(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_DATETIME_FORMAT_UTC: WS_DATETIME_FORMAT = WS_DATETIME_FORMAT(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_DATETIME_FORMAT_LOCAL: WS_DATETIME_FORMAT = WS_DATETIME_FORMAT(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_DATETIME_FORMAT_NONE: WS_DATETIME_FORMAT = WS_DATETIME_FORMAT(2i32);
impl ::core::marker::Copy for WS_DATETIME_FORMAT {}
impl ::core::clone::Clone for WS_DATETIME_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_DATETIME_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_DATETIME_FORMAT {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_DATETIME_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_DATETIME_FORMAT").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_DECIMAL_DESCRIPTION {
    pub minValue: super::super::Foundation::DECIMAL,
    pub maxValue: super::super::Foundation::DECIMAL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_DECIMAL_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_DECIMAL_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_DECIMAL_DESCRIPTION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_DECIMAL_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_DECIMAL_DESCRIPTION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_DECIMAL_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_DECIMAL_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_DECODER_DECODE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(encodercontext: *const ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void, maxlength: u32, length: *mut u32, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_DECODER_END_CALLBACK = ::core::option::Option<unsafe extern "system" fn(encodercontext: *const ::core::ffi::c_void, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_DECODER_GET_CONTENT_TYPE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(decodercontext: *const ::core::ffi::c_void, contenttype: *const WS_STRING, contentencoding: *const WS_STRING, newcontenttype: *mut WS_STRING, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_DECODER_START_CALLBACK = ::core::option::Option<unsafe extern "system" fn(encodercontext: *const ::core::ffi::c_void, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_DEFAULT_VALUE {
    pub value: *mut ::core::ffi::c_void,
    pub valueSize: u32,
}
impl ::core::marker::Copy for WS_DEFAULT_VALUE {}
impl ::core::clone::Clone for WS_DEFAULT_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_DEFAULT_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_DEFAULT_VALUE").field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_DEFAULT_VALUE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_DEFAULT_VALUE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_DEFAULT_VALUE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_DEFAULT_VALUE {}
impl ::core::default::Default for WS_DEFAULT_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_DEFAULT_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    pub credential: WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL,
}
impl ::core::marker::Copy for WS_DEFAULT_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {}
impl ::core::clone::Clone for WS_DEFAULT_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_DEFAULT_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_DEFAULT_WINDOWS_INTEGRATED_AUTH_CREDENTIAL").field("credential", &self.credential).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_DEFAULT_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_DEFAULT_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_DEFAULT_WINDOWS_INTEGRATED_AUTH_CREDENTIAL>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_DEFAULT_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {}
impl ::core::default::Default for WS_DEFAULT_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_DISALLOWED_USER_AGENT_SUBSTRINGS {
    pub subStringCount: u32,
    pub subStrings: *mut *mut WS_STRING,
}
impl ::core::marker::Copy for WS_DISALLOWED_USER_AGENT_SUBSTRINGS {}
impl ::core::clone::Clone for WS_DISALLOWED_USER_AGENT_SUBSTRINGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_DISALLOWED_USER_AGENT_SUBSTRINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_DISALLOWED_USER_AGENT_SUBSTRINGS").field("subStringCount", &self.subStringCount).field("subStrings", &self.subStrings).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_DISALLOWED_USER_AGENT_SUBSTRINGS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_DISALLOWED_USER_AGENT_SUBSTRINGS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_DISALLOWED_USER_AGENT_SUBSTRINGS>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_DISALLOWED_USER_AGENT_SUBSTRINGS {}
impl ::core::default::Default for WS_DISALLOWED_USER_AGENT_SUBSTRINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_DNS_ENDPOINT_IDENTITY {
    pub identity: WS_ENDPOINT_IDENTITY,
    pub dns: WS_STRING,
}
impl ::core::marker::Copy for WS_DNS_ENDPOINT_IDENTITY {}
impl ::core::clone::Clone for WS_DNS_ENDPOINT_IDENTITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_DNS_ENDPOINT_IDENTITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_DNS_ENDPOINT_IDENTITY").field("identity", &self.identity).field("dns", &self.dns).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_DNS_ENDPOINT_IDENTITY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_DNS_ENDPOINT_IDENTITY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_DNS_ENDPOINT_IDENTITY>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_DNS_ENDPOINT_IDENTITY {}
impl ::core::default::Default for WS_DNS_ENDPOINT_IDENTITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_DOUBLE_DESCRIPTION {
    pub minValue: f64,
    pub maxValue: f64,
}
impl ::core::marker::Copy for WS_DOUBLE_DESCRIPTION {}
impl ::core::clone::Clone for WS_DOUBLE_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_DOUBLE_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_DOUBLE_DESCRIPTION").field("minValue", &self.minValue).field("maxValue", &self.maxValue).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_DOUBLE_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_DOUBLE_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_DOUBLE_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_DOUBLE_DESCRIPTION {}
impl ::core::default::Default for WS_DOUBLE_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_DURATION {
    pub negative: super::super::Foundation::BOOL,
    pub years: u32,
    pub months: u32,
    pub days: u32,
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
    pub milliseconds: u32,
    pub ticks: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_DURATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_DURATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_DURATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_DURATION").field("negative", &self.negative).field("years", &self.years).field("months", &self.months).field("days", &self.days).field("hours", &self.hours).field("minutes", &self.minutes).field("seconds", &self.seconds).field("milliseconds", &self.milliseconds).field("ticks", &self.ticks).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_DURATION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_DURATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_DURATION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_DURATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_DURATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type WS_DURATION_COMPARISON_CALLBACK = ::core::option::Option<unsafe extern "system" fn(duration1: *const WS_DURATION, duration2: *const WS_DURATION, result: *mut i32, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_DURATION_DESCRIPTION {
    pub minValue: WS_DURATION,
    pub maxValue: WS_DURATION,
    pub comparer: WS_DURATION_COMPARISON_CALLBACK,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_DURATION_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_DURATION_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_DURATION_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_DURATION_DESCRIPTION").field("minValue", &self.minValue).field("maxValue", &self.maxValue).field("comparer", &self.comparer.map(|f| f as usize)).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_DURATION_DESCRIPTION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_DURATION_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_DURATION_DESCRIPTION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_DURATION_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_DURATION_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type WS_DYNAMIC_STRING_CALLBACK = ::core::option::Option<unsafe extern "system" fn(callbackstate: *const ::core::ffi::c_void, string: *const WS_XML_STRING, found: *mut super::super::Foundation::BOOL, id: *mut u32, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_ELEMENT_DESCRIPTION {
    pub elementLocalName: *mut WS_XML_STRING,
    pub elementNs: *mut WS_XML_STRING,
    pub r#type: WS_TYPE,
    pub typeDescription: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_ELEMENT_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_ELEMENT_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_ELEMENT_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ELEMENT_DESCRIPTION").field("elementLocalName", &self.elementLocalName).field("elementNs", &self.elementNs).field("type", &self.r#type).field("typeDescription", &self.typeDescription).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_ELEMENT_DESCRIPTION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_ELEMENT_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_ELEMENT_DESCRIPTION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_ELEMENT_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_ELEMENT_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_ENCODER_ENCODE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(encodercontext: *const ::core::ffi::c_void, buffers: *const WS_BYTES, count: u32, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_ENCODER_END_CALLBACK = ::core::option::Option<unsafe extern "system" fn(encodercontext: *const ::core::ffi::c_void, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_ENCODER_GET_CONTENT_TYPE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(encodercontext: *const ::core::ffi::c_void, contenttype: *const WS_STRING, newcontenttype: *mut WS_STRING, contentencoding: *mut WS_STRING, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_ENCODER_START_CALLBACK = ::core::option::Option<unsafe extern "system" fn(encodercontext: *const ::core::ffi::c_void, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_ENCODING(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ENCODING_XML_BINARY_1: WS_ENCODING = WS_ENCODING(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ENCODING_XML_BINARY_SESSION_1: WS_ENCODING = WS_ENCODING(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ENCODING_XML_MTOM_UTF8: WS_ENCODING = WS_ENCODING(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ENCODING_XML_MTOM_UTF16BE: WS_ENCODING = WS_ENCODING(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ENCODING_XML_MTOM_UTF16LE: WS_ENCODING = WS_ENCODING(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ENCODING_XML_UTF8: WS_ENCODING = WS_ENCODING(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ENCODING_XML_UTF16BE: WS_ENCODING = WS_ENCODING(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ENCODING_XML_UTF16LE: WS_ENCODING = WS_ENCODING(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ENCODING_RAW: WS_ENCODING = WS_ENCODING(8i32);
impl ::core::marker::Copy for WS_ENCODING {}
impl ::core::clone::Clone for WS_ENCODING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_ENCODING {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_ENCODING {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_ENCODING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_ENCODING").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_ENDPOINT_ADDRESS {
    pub url: WS_STRING,
    pub headers: *mut WS_XML_BUFFER,
    pub extensions: *mut WS_XML_BUFFER,
    pub identity: *mut WS_ENDPOINT_IDENTITY,
}
impl ::core::marker::Copy for WS_ENDPOINT_ADDRESS {}
impl ::core::clone::Clone for WS_ENDPOINT_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_ENDPOINT_ADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ENDPOINT_ADDRESS").field("url", &self.url).field("headers", &self.headers).field("extensions", &self.extensions).field("identity", &self.identity).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_ENDPOINT_ADDRESS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_ENDPOINT_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_ENDPOINT_ADDRESS>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_ENDPOINT_ADDRESS {}
impl ::core::default::Default for WS_ENDPOINT_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_ENDPOINT_ADDRESS_DESCRIPTION {
    pub addressingVersion: WS_ADDRESSING_VERSION,
}
impl ::core::marker::Copy for WS_ENDPOINT_ADDRESS_DESCRIPTION {}
impl ::core::clone::Clone for WS_ENDPOINT_ADDRESS_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_ENDPOINT_ADDRESS_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ENDPOINT_ADDRESS_DESCRIPTION").field("addressingVersion", &self.addressingVersion).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_ENDPOINT_ADDRESS_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_ENDPOINT_ADDRESS_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_ENDPOINT_ADDRESS_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_ENDPOINT_ADDRESS_DESCRIPTION {}
impl ::core::default::Default for WS_ENDPOINT_ADDRESS_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_ENDPOINT_ADDRESS_EXTENSION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ENDPOINT_ADDRESS_EXTENSION_METADATA_ADDRESS: WS_ENDPOINT_ADDRESS_EXTENSION_TYPE = WS_ENDPOINT_ADDRESS_EXTENSION_TYPE(1i32);
impl ::core::marker::Copy for WS_ENDPOINT_ADDRESS_EXTENSION_TYPE {}
impl ::core::clone::Clone for WS_ENDPOINT_ADDRESS_EXTENSION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_ENDPOINT_ADDRESS_EXTENSION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_ENDPOINT_ADDRESS_EXTENSION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_ENDPOINT_ADDRESS_EXTENSION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_ENDPOINT_ADDRESS_EXTENSION_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_ENDPOINT_IDENTITY {
    pub identityType: WS_ENDPOINT_IDENTITY_TYPE,
}
impl ::core::marker::Copy for WS_ENDPOINT_IDENTITY {}
impl ::core::clone::Clone for WS_ENDPOINT_IDENTITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_ENDPOINT_IDENTITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ENDPOINT_IDENTITY").field("identityType", &self.identityType).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_ENDPOINT_IDENTITY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_ENDPOINT_IDENTITY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_ENDPOINT_IDENTITY>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_ENDPOINT_IDENTITY {}
impl ::core::default::Default for WS_ENDPOINT_IDENTITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_ENDPOINT_IDENTITY_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_DNS_ENDPOINT_IDENTITY_TYPE: WS_ENDPOINT_IDENTITY_TYPE = WS_ENDPOINT_IDENTITY_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_UPN_ENDPOINT_IDENTITY_TYPE: WS_ENDPOINT_IDENTITY_TYPE = WS_ENDPOINT_IDENTITY_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SPN_ENDPOINT_IDENTITY_TYPE: WS_ENDPOINT_IDENTITY_TYPE = WS_ENDPOINT_IDENTITY_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_RSA_ENDPOINT_IDENTITY_TYPE: WS_ENDPOINT_IDENTITY_TYPE = WS_ENDPOINT_IDENTITY_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CERT_ENDPOINT_IDENTITY_TYPE: WS_ENDPOINT_IDENTITY_TYPE = WS_ENDPOINT_IDENTITY_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_UNKNOWN_ENDPOINT_IDENTITY_TYPE: WS_ENDPOINT_IDENTITY_TYPE = WS_ENDPOINT_IDENTITY_TYPE(6i32);
impl ::core::marker::Copy for WS_ENDPOINT_IDENTITY_TYPE {}
impl ::core::clone::Clone for WS_ENDPOINT_IDENTITY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_ENDPOINT_IDENTITY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_ENDPOINT_IDENTITY_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_ENDPOINT_IDENTITY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_ENDPOINT_IDENTITY_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_ENDPOINT_POLICY_EXTENSION {
    pub policyExtension: WS_POLICY_EXTENSION,
    pub assertionName: *mut WS_XML_STRING,
    pub assertionNs: *mut WS_XML_STRING,
    pub out: WS_ENDPOINT_POLICY_EXTENSION_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_ENDPOINT_POLICY_EXTENSION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_ENDPOINT_POLICY_EXTENSION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_ENDPOINT_POLICY_EXTENSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ENDPOINT_POLICY_EXTENSION").field("policyExtension", &self.policyExtension).field("assertionName", &self.assertionName).field("assertionNs", &self.assertionNs).field("out", &self.out).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_ENDPOINT_POLICY_EXTENSION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_ENDPOINT_POLICY_EXTENSION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_ENDPOINT_POLICY_EXTENSION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_ENDPOINT_POLICY_EXTENSION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_ENDPOINT_POLICY_EXTENSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_ENDPOINT_POLICY_EXTENSION_0 {
    pub assertionValue: *mut WS_XML_BUFFER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_ENDPOINT_POLICY_EXTENSION_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_ENDPOINT_POLICY_EXTENSION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_ENDPOINT_POLICY_EXTENSION_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ENDPOINT_POLICY_EXTENSION_0").field("assertionValue", &self.assertionValue).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_ENDPOINT_POLICY_EXTENSION_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_ENDPOINT_POLICY_EXTENSION_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_ENDPOINT_POLICY_EXTENSION_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_ENDPOINT_POLICY_EXTENSION_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_ENDPOINT_POLICY_EXTENSION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_ENUM_DESCRIPTION {
    pub values: *mut WS_ENUM_VALUE,
    pub valueCount: u32,
    pub maxByteCount: u32,
    pub nameIndices: *mut u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_ENUM_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_ENUM_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_ENUM_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ENUM_DESCRIPTION").field("values", &self.values).field("valueCount", &self.valueCount).field("maxByteCount", &self.maxByteCount).field("nameIndices", &self.nameIndices).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_ENUM_DESCRIPTION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_ENUM_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_ENUM_DESCRIPTION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_ENUM_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_ENUM_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_ENUM_VALUE {
    pub value: i32,
    pub name: *mut WS_XML_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_ENUM_VALUE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_ENUM_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_ENUM_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ENUM_VALUE").field("value", &self.value).field("name", &self.name).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_ENUM_VALUE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_ENUM_VALUE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_ENUM_VALUE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_ENUM_VALUE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_ENUM_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_ENVELOPE_VERSION(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ENVELOPE_VERSION_SOAP_1_1: WS_ENVELOPE_VERSION = WS_ENVELOPE_VERSION(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ENVELOPE_VERSION_SOAP_1_2: WS_ENVELOPE_VERSION = WS_ENVELOPE_VERSION(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ENVELOPE_VERSION_NONE: WS_ENVELOPE_VERSION = WS_ENVELOPE_VERSION(3i32);
impl ::core::marker::Copy for WS_ENVELOPE_VERSION {}
impl ::core::clone::Clone for WS_ENVELOPE_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_ENVELOPE_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_ENVELOPE_VERSION {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_ENVELOPE_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_ENVELOPE_VERSION").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct WS_ERROR(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_ERROR_PROPERTY {
    pub id: WS_ERROR_PROPERTY_ID,
    pub value: *mut ::core::ffi::c_void,
    pub valueSize: u32,
}
impl ::core::marker::Copy for WS_ERROR_PROPERTY {}
impl ::core::clone::Clone for WS_ERROR_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_ERROR_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ERROR_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_ERROR_PROPERTY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_ERROR_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_ERROR_PROPERTY>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_ERROR_PROPERTY {}
impl ::core::default::Default for WS_ERROR_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_ERROR_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ERROR_PROPERTY_STRING_COUNT: WS_ERROR_PROPERTY_ID = WS_ERROR_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ERROR_PROPERTY_ORIGINAL_ERROR_CODE: WS_ERROR_PROPERTY_ID = WS_ERROR_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ERROR_PROPERTY_LANGID: WS_ERROR_PROPERTY_ID = WS_ERROR_PROPERTY_ID(2i32);
impl ::core::marker::Copy for WS_ERROR_PROPERTY_ID {}
impl ::core::clone::Clone for WS_ERROR_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_ERROR_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_ERROR_PROPERTY_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_ERROR_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_ERROR_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_EXCEPTION_CODE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_EXCEPTION_CODE_USAGE_FAILURE: WS_EXCEPTION_CODE = WS_EXCEPTION_CODE(-1069744128i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_EXCEPTION_CODE_INTERNAL_FAILURE: WS_EXCEPTION_CODE = WS_EXCEPTION_CODE(-1069744127i32);
impl ::core::marker::Copy for WS_EXCEPTION_CODE {}
impl ::core::clone::Clone for WS_EXCEPTION_CODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_EXCEPTION_CODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_EXCEPTION_CODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_EXCEPTION_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_EXCEPTION_CODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_EXTENDED_PROTECTION_POLICY(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_EXTENDED_PROTECTION_POLICY_NEVER: WS_EXTENDED_PROTECTION_POLICY = WS_EXTENDED_PROTECTION_POLICY(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_EXTENDED_PROTECTION_POLICY_WHEN_SUPPORTED: WS_EXTENDED_PROTECTION_POLICY = WS_EXTENDED_PROTECTION_POLICY(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_EXTENDED_PROTECTION_POLICY_ALWAYS: WS_EXTENDED_PROTECTION_POLICY = WS_EXTENDED_PROTECTION_POLICY(3i32);
impl ::core::marker::Copy for WS_EXTENDED_PROTECTION_POLICY {}
impl ::core::clone::Clone for WS_EXTENDED_PROTECTION_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_EXTENDED_PROTECTION_POLICY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_EXTENDED_PROTECTION_POLICY {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_EXTENDED_PROTECTION_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_EXTENDED_PROTECTION_POLICY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_EXTENDED_PROTECTION_SCENARIO(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_EXTENDED_PROTECTION_SCENARIO_BOUND_SERVER: WS_EXTENDED_PROTECTION_SCENARIO = WS_EXTENDED_PROTECTION_SCENARIO(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_EXTENDED_PROTECTION_SCENARIO_TERMINATED_SSL: WS_EXTENDED_PROTECTION_SCENARIO = WS_EXTENDED_PROTECTION_SCENARIO(2i32);
impl ::core::marker::Copy for WS_EXTENDED_PROTECTION_SCENARIO {}
impl ::core::clone::Clone for WS_EXTENDED_PROTECTION_SCENARIO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_EXTENDED_PROTECTION_SCENARIO {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_EXTENDED_PROTECTION_SCENARIO {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_EXTENDED_PROTECTION_SCENARIO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_EXTENDED_PROTECTION_SCENARIO").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_FAULT {
    pub code: *mut WS_FAULT_CODE,
    pub reasons: *mut WS_FAULT_REASON,
    pub reasonCount: u32,
    pub actor: WS_STRING,
    pub node: WS_STRING,
    pub detail: *mut WS_XML_BUFFER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_FAULT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_FAULT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_FAULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_FAULT").field("code", &self.code).field("reasons", &self.reasons).field("reasonCount", &self.reasonCount).field("actor", &self.actor).field("node", &self.node).field("detail", &self.detail).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_FAULT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_FAULT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_FAULT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_FAULT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_FAULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_FAULT_CODE {
    pub value: WS_XML_QNAME,
    pub subCode: *mut WS_FAULT_CODE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_FAULT_CODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_FAULT_CODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_FAULT_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_FAULT_CODE").field("value", &self.value).field("subCode", &self.subCode).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_FAULT_CODE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_FAULT_CODE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_FAULT_CODE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_FAULT_CODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_FAULT_CODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_FAULT_DESCRIPTION {
    pub envelopeVersion: WS_ENVELOPE_VERSION,
}
impl ::core::marker::Copy for WS_FAULT_DESCRIPTION {}
impl ::core::clone::Clone for WS_FAULT_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_FAULT_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_FAULT_DESCRIPTION").field("envelopeVersion", &self.envelopeVersion).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_FAULT_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_FAULT_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_FAULT_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_FAULT_DESCRIPTION {}
impl ::core::default::Default for WS_FAULT_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_FAULT_DETAIL_DESCRIPTION {
    pub action: *mut WS_XML_STRING,
    pub detailElementDescription: *mut WS_ELEMENT_DESCRIPTION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_FAULT_DETAIL_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_FAULT_DETAIL_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_FAULT_DETAIL_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_FAULT_DETAIL_DESCRIPTION").field("action", &self.action).field("detailElementDescription", &self.detailElementDescription).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_FAULT_DETAIL_DESCRIPTION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_FAULT_DETAIL_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_FAULT_DETAIL_DESCRIPTION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_FAULT_DETAIL_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_FAULT_DETAIL_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_FAULT_DISCLOSURE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MINIMAL_FAULT_DISCLOSURE: WS_FAULT_DISCLOSURE = WS_FAULT_DISCLOSURE(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_FULL_FAULT_DISCLOSURE: WS_FAULT_DISCLOSURE = WS_FAULT_DISCLOSURE(1i32);
impl ::core::marker::Copy for WS_FAULT_DISCLOSURE {}
impl ::core::clone::Clone for WS_FAULT_DISCLOSURE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_FAULT_DISCLOSURE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_FAULT_DISCLOSURE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_FAULT_DISCLOSURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_FAULT_DISCLOSURE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_FAULT_ERROR_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_FAULT_ERROR_PROPERTY_FAULT: WS_FAULT_ERROR_PROPERTY_ID = WS_FAULT_ERROR_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_FAULT_ERROR_PROPERTY_ACTION: WS_FAULT_ERROR_PROPERTY_ID = WS_FAULT_ERROR_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_FAULT_ERROR_PROPERTY_HEADER: WS_FAULT_ERROR_PROPERTY_ID = WS_FAULT_ERROR_PROPERTY_ID(2i32);
impl ::core::marker::Copy for WS_FAULT_ERROR_PROPERTY_ID {}
impl ::core::clone::Clone for WS_FAULT_ERROR_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_FAULT_ERROR_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_FAULT_ERROR_PROPERTY_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_FAULT_ERROR_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_FAULT_ERROR_PROPERTY_ID").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_FAULT_REASON {
    pub text: WS_STRING,
    pub lang: WS_STRING,
}
impl ::core::marker::Copy for WS_FAULT_REASON {}
impl ::core::clone::Clone for WS_FAULT_REASON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_FAULT_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_FAULT_REASON").field("text", &self.text).field("lang", &self.lang).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_FAULT_REASON {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_FAULT_REASON {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_FAULT_REASON>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_FAULT_REASON {}
impl ::core::default::Default for WS_FAULT_REASON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_FIELD_DESCRIPTION {
    pub mapping: WS_FIELD_MAPPING,
    pub localName: *mut WS_XML_STRING,
    pub ns: *mut WS_XML_STRING,
    pub r#type: WS_TYPE,
    pub typeDescription: *mut ::core::ffi::c_void,
    pub offset: u32,
    pub options: u32,
    pub defaultValue: *mut WS_DEFAULT_VALUE,
    pub countOffset: u32,
    pub itemLocalName: *mut WS_XML_STRING,
    pub itemNs: *mut WS_XML_STRING,
    pub itemRange: *mut WS_ITEM_RANGE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_FIELD_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_FIELD_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_FIELD_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_FIELD_DESCRIPTION").field("mapping", &self.mapping).field("localName", &self.localName).field("ns", &self.ns).field("type", &self.r#type).field("typeDescription", &self.typeDescription).field("offset", &self.offset).field("options", &self.options).field("defaultValue", &self.defaultValue).field("countOffset", &self.countOffset).field("itemLocalName", &self.itemLocalName).field("itemNs", &self.itemNs).field("itemRange", &self.itemRange).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_FIELD_DESCRIPTION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_FIELD_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_FIELD_DESCRIPTION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_FIELD_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_FIELD_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_FIELD_MAPPING(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TYPE_ATTRIBUTE_FIELD_MAPPING: WS_FIELD_MAPPING = WS_FIELD_MAPPING(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ATTRIBUTE_FIELD_MAPPING: WS_FIELD_MAPPING = WS_FIELD_MAPPING(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ELEMENT_FIELD_MAPPING: WS_FIELD_MAPPING = WS_FIELD_MAPPING(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_REPEATING_ELEMENT_FIELD_MAPPING: WS_FIELD_MAPPING = WS_FIELD_MAPPING(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TEXT_FIELD_MAPPING: WS_FIELD_MAPPING = WS_FIELD_MAPPING(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_NO_FIELD_MAPPING: WS_FIELD_MAPPING = WS_FIELD_MAPPING(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_ATTRIBUTE_FIELD_MAPPING: WS_FIELD_MAPPING = WS_FIELD_MAPPING(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ELEMENT_CHOICE_FIELD_MAPPING: WS_FIELD_MAPPING = WS_FIELD_MAPPING(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_REPEATING_ELEMENT_CHOICE_FIELD_MAPPING: WS_FIELD_MAPPING = WS_FIELD_MAPPING(8i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ANY_ELEMENT_FIELD_MAPPING: WS_FIELD_MAPPING = WS_FIELD_MAPPING(9i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_REPEATING_ANY_ELEMENT_FIELD_MAPPING: WS_FIELD_MAPPING = WS_FIELD_MAPPING(10i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ANY_CONTENT_FIELD_MAPPING: WS_FIELD_MAPPING = WS_FIELD_MAPPING(11i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ANY_ATTRIBUTES_FIELD_MAPPING: WS_FIELD_MAPPING = WS_FIELD_MAPPING(12i32);
impl ::core::marker::Copy for WS_FIELD_MAPPING {}
impl ::core::clone::Clone for WS_FIELD_MAPPING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_FIELD_MAPPING {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_FIELD_MAPPING {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_FIELD_MAPPING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_FIELD_MAPPING").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_FIELD_NILLABLE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_FIELD_NILLABLE_ITEM: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_FIELD_OPTIONAL: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_FIELD_OTHER_NAMESPACE: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_FIELD_POINTER: i32 = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_FLOAT_DESCRIPTION {
    pub minValue: f32,
    pub maxValue: f32,
}
impl ::core::marker::Copy for WS_FLOAT_DESCRIPTION {}
impl ::core::clone::Clone for WS_FLOAT_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_FLOAT_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_FLOAT_DESCRIPTION").field("minValue", &self.minValue).field("maxValue", &self.maxValue).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_FLOAT_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_FLOAT_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_FLOAT_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_FLOAT_DESCRIPTION {}
impl ::core::default::Default for WS_FLOAT_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_FREE_CHANNEL_CALLBACK = ::core::option::Option<unsafe extern "system" fn(channelinstance: *const ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_FREE_DECODER_CALLBACK = ::core::option::Option<unsafe extern "system" fn(decodercontext: *const ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_FREE_ENCODER_CALLBACK = ::core::option::Option<unsafe extern "system" fn(encodercontext: *const ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_FREE_LISTENER_CALLBACK = ::core::option::Option<unsafe extern "system" fn(listenerinstance: *const ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub type WS_GET_CERT_CALLBACK = ::core::option::Option<unsafe extern "system" fn(getcertcallbackstate: *const ::core::ffi::c_void, targetaddress: *const WS_ENDPOINT_ADDRESS, viauri: *const WS_STRING, cert: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_GET_CHANNEL_PROPERTY_CALLBACK = ::core::option::Option<unsafe extern "system" fn(channelinstance: *const ::core::ffi::c_void, id: WS_CHANNEL_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_GET_LISTENER_PROPERTY_CALLBACK = ::core::option::Option<unsafe extern "system" fn(listenerinstance: *const ::core::ffi::c_void, id: WS_LISTENER_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_GUID_DESCRIPTION {
    pub value: ::windows::core::GUID,
}
impl ::core::marker::Copy for WS_GUID_DESCRIPTION {}
impl ::core::clone::Clone for WS_GUID_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_GUID_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_GUID_DESCRIPTION").field("value", &self.value).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_GUID_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_GUID_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_GUID_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_GUID_DESCRIPTION {}
impl ::core::default::Default for WS_GUID_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_HEADER_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ACTION_HEADER: WS_HEADER_TYPE = WS_HEADER_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TO_HEADER: WS_HEADER_TYPE = WS_HEADER_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_ID_HEADER: WS_HEADER_TYPE = WS_HEADER_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_RELATES_TO_HEADER: WS_HEADER_TYPE = WS_HEADER_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_FROM_HEADER: WS_HEADER_TYPE = WS_HEADER_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_REPLY_TO_HEADER: WS_HEADER_TYPE = WS_HEADER_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_FAULT_TO_HEADER: WS_HEADER_TYPE = WS_HEADER_TYPE(7i32);
impl ::core::marker::Copy for WS_HEADER_TYPE {}
impl ::core::clone::Clone for WS_HEADER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_HEADER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_HEADER_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_HEADER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_HEADER_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct WS_HEAP(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HEAP_PROPERTIES {
    pub properties: *mut WS_HEAP_PROPERTY,
    pub propertyCount: u32,
}
impl ::core::marker::Copy for WS_HEAP_PROPERTIES {}
impl ::core::clone::Clone for WS_HEAP_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HEAP_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HEAP_PROPERTIES").field("properties", &self.properties).field("propertyCount", &self.propertyCount).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_HEAP_PROPERTIES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_HEAP_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_HEAP_PROPERTIES>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_HEAP_PROPERTIES {}
impl ::core::default::Default for WS_HEAP_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HEAP_PROPERTY {
    pub id: WS_HEAP_PROPERTY_ID,
    pub value: *mut ::core::ffi::c_void,
    pub valueSize: u32,
}
impl ::core::marker::Copy for WS_HEAP_PROPERTY {}
impl ::core::clone::Clone for WS_HEAP_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HEAP_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HEAP_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_HEAP_PROPERTY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_HEAP_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_HEAP_PROPERTY>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_HEAP_PROPERTY {}
impl ::core::default::Default for WS_HEAP_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_HEAP_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HEAP_PROPERTY_MAX_SIZE: WS_HEAP_PROPERTY_ID = WS_HEAP_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HEAP_PROPERTY_TRIM_SIZE: WS_HEAP_PROPERTY_ID = WS_HEAP_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HEAP_PROPERTY_REQUESTED_SIZE: WS_HEAP_PROPERTY_ID = WS_HEAP_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HEAP_PROPERTY_ACTUAL_SIZE: WS_HEAP_PROPERTY_ID = WS_HEAP_PROPERTY_ID(3i32);
impl ::core::marker::Copy for WS_HEAP_PROPERTY_ID {}
impl ::core::clone::Clone for WS_HEAP_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_HEAP_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_HEAP_PROPERTY_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_HEAP_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_HEAP_PROPERTY_ID").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HOST_NAMES {
    pub hostNames: *mut WS_STRING,
    pub hostNameCount: u32,
}
impl ::core::marker::Copy for WS_HOST_NAMES {}
impl ::core::clone::Clone for WS_HOST_NAMES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HOST_NAMES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HOST_NAMES").field("hostNames", &self.hostNames).field("hostNameCount", &self.hostNameCount).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_HOST_NAMES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_HOST_NAMES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_HOST_NAMES>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_HOST_NAMES {}
impl ::core::default::Default for WS_HOST_NAMES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HTTPS_URL {
    pub url: WS_URL,
    pub host: WS_STRING,
    pub port: u16,
    pub portAsString: WS_STRING,
    pub path: WS_STRING,
    pub query: WS_STRING,
    pub fragment: WS_STRING,
}
impl ::core::marker::Copy for WS_HTTPS_URL {}
impl ::core::clone::Clone for WS_HTTPS_URL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HTTPS_URL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTPS_URL").field("url", &self.url).field("host", &self.host).field("port", &self.port).field("portAsString", &self.portAsString).field("path", &self.path).field("query", &self.query).field("fragment", &self.fragment).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_HTTPS_URL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_HTTPS_URL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_HTTPS_URL>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_HTTPS_URL {}
impl ::core::default::Default for WS_HTTPS_URL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HTTP_BINDING_TEMPLATE {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
}
impl ::core::marker::Copy for WS_HTTP_BINDING_TEMPLATE {}
impl ::core::clone::Clone for WS_HTTP_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HTTP_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_BINDING_TEMPLATE").field("channelProperties", &self.channelProperties).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_HTTP_BINDING_TEMPLATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_HTTP_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_HTTP_BINDING_TEMPLATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_HTTP_BINDING_TEMPLATE {}
impl ::core::default::Default for WS_HTTP_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HTTP_HEADER_AUTH_BINDING_TEMPLATE {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub httpHeaderAuthSecurityBinding: WS_HTTP_HEADER_AUTH_SECURITY_BINDING_TEMPLATE,
}
impl ::core::marker::Copy for WS_HTTP_HEADER_AUTH_BINDING_TEMPLATE {}
impl ::core::clone::Clone for WS_HTTP_HEADER_AUTH_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HTTP_HEADER_AUTH_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_HEADER_AUTH_BINDING_TEMPLATE").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("httpHeaderAuthSecurityBinding", &self.httpHeaderAuthSecurityBinding).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_HTTP_HEADER_AUTH_BINDING_TEMPLATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_HTTP_HEADER_AUTH_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_HTTP_HEADER_AUTH_BINDING_TEMPLATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_HTTP_HEADER_AUTH_BINDING_TEMPLATE {}
impl ::core::default::Default for WS_HTTP_HEADER_AUTH_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HTTP_HEADER_AUTH_POLICY_DESCRIPTION {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub httpHeaderAuthSecurityBinding: WS_HTTP_HEADER_AUTH_SECURITY_BINDING_POLICY_DESCRIPTION,
}
impl ::core::marker::Copy for WS_HTTP_HEADER_AUTH_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_HTTP_HEADER_AUTH_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HTTP_HEADER_AUTH_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_HEADER_AUTH_POLICY_DESCRIPTION").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("httpHeaderAuthSecurityBinding", &self.httpHeaderAuthSecurityBinding).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_HTTP_HEADER_AUTH_POLICY_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_HTTP_HEADER_AUTH_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_HTTP_HEADER_AUTH_POLICY_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_HTTP_HEADER_AUTH_POLICY_DESCRIPTION {}
impl ::core::default::Default for WS_HTTP_HEADER_AUTH_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_HEADER_AUTH_SCHEME_BASIC: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_HEADER_AUTH_SCHEME_DIGEST: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_HEADER_AUTH_SCHEME_NEGOTIATE: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_HEADER_AUTH_SCHEME_NONE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_HEADER_AUTH_SCHEME_NTLM: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_HEADER_AUTH_SCHEME_PASSPORT: i32 = 32i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HTTP_HEADER_AUTH_SECURITY_BINDING {
    pub binding: WS_SECURITY_BINDING,
    pub clientCredential: *mut WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL,
}
impl ::core::marker::Copy for WS_HTTP_HEADER_AUTH_SECURITY_BINDING {}
impl ::core::clone::Clone for WS_HTTP_HEADER_AUTH_SECURITY_BINDING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HTTP_HEADER_AUTH_SECURITY_BINDING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_HEADER_AUTH_SECURITY_BINDING").field("binding", &self.binding).field("clientCredential", &self.clientCredential).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_HTTP_HEADER_AUTH_SECURITY_BINDING {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_HTTP_HEADER_AUTH_SECURITY_BINDING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_HTTP_HEADER_AUTH_SECURITY_BINDING>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_HTTP_HEADER_AUTH_SECURITY_BINDING {}
impl ::core::default::Default for WS_HTTP_HEADER_AUTH_SECURITY_BINDING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HTTP_HEADER_AUTH_SECURITY_BINDING_CONSTRAINT {
    pub bindingConstraint: WS_SECURITY_BINDING_CONSTRAINT,
}
impl ::core::marker::Copy for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_CONSTRAINT {}
impl ::core::clone::Clone for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_CONSTRAINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_CONSTRAINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_HEADER_AUTH_SECURITY_BINDING_CONSTRAINT").field("bindingConstraint", &self.bindingConstraint).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_CONSTRAINT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_CONSTRAINT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_HTTP_HEADER_AUTH_SECURITY_BINDING_CONSTRAINT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_CONSTRAINT {}
impl ::core::default::Default for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_CONSTRAINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HTTP_HEADER_AUTH_SECURITY_BINDING_POLICY_DESCRIPTION {
    pub securityBindingProperties: WS_SECURITY_BINDING_PROPERTIES,
}
impl ::core::marker::Copy for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_HEADER_AUTH_SECURITY_BINDING_POLICY_DESCRIPTION").field("securityBindingProperties", &self.securityBindingProperties).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_POLICY_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_HTTP_HEADER_AUTH_SECURITY_BINDING_POLICY_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_POLICY_DESCRIPTION {}
impl ::core::default::Default for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HTTP_HEADER_AUTH_SECURITY_BINDING_TEMPLATE {
    pub securityBindingProperties: WS_SECURITY_BINDING_PROPERTIES,
    pub clientCredential: *mut WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL,
}
impl ::core::marker::Copy for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_TEMPLATE {}
impl ::core::clone::Clone for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_HEADER_AUTH_SECURITY_BINDING_TEMPLATE").field("securityBindingProperties", &self.securityBindingProperties).field("clientCredential", &self.clientCredential).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_TEMPLATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_HTTP_HEADER_AUTH_SECURITY_BINDING_TEMPLATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_TEMPLATE {}
impl ::core::default::Default for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_HTTP_HEADER_AUTH_TARGET(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_HEADER_AUTH_TARGET_SERVICE: WS_HTTP_HEADER_AUTH_TARGET = WS_HTTP_HEADER_AUTH_TARGET(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_HEADER_AUTH_TARGET_PROXY: WS_HTTP_HEADER_AUTH_TARGET = WS_HTTP_HEADER_AUTH_TARGET(2i32);
impl ::core::marker::Copy for WS_HTTP_HEADER_AUTH_TARGET {}
impl ::core::clone::Clone for WS_HTTP_HEADER_AUTH_TARGET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_HTTP_HEADER_AUTH_TARGET {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_HTTP_HEADER_AUTH_TARGET {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_HTTP_HEADER_AUTH_TARGET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_HTTP_HEADER_AUTH_TARGET").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_HTTP_HEADER_MAPPING {
    pub headerName: WS_XML_STRING,
    pub headerMappingOptions: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_HTTP_HEADER_MAPPING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_HTTP_HEADER_MAPPING {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_HTTP_HEADER_MAPPING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_HEADER_MAPPING").field("headerName", &self.headerName).field("headerMappingOptions", &self.headerMappingOptions).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_HTTP_HEADER_MAPPING {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_HTTP_HEADER_MAPPING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_HTTP_HEADER_MAPPING>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_HTTP_HEADER_MAPPING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_HTTP_HEADER_MAPPING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_HEADER_MAPPING_COMMA_SEPARATOR: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_HEADER_MAPPING_QUOTED_VALUE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_HEADER_MAPPING_SEMICOLON_SEPARATOR: i32 = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_HTTP_MESSAGE_MAPPING {
    pub requestMappingOptions: u32,
    pub responseMappingOptions: u32,
    pub requestHeaderMappings: *mut *mut WS_HTTP_HEADER_MAPPING,
    pub requestHeaderMappingCount: u32,
    pub responseHeaderMappings: *mut *mut WS_HTTP_HEADER_MAPPING,
    pub responseHeaderMappingCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_HTTP_MESSAGE_MAPPING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_HTTP_MESSAGE_MAPPING {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_HTTP_MESSAGE_MAPPING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_MESSAGE_MAPPING").field("requestMappingOptions", &self.requestMappingOptions).field("responseMappingOptions", &self.responseMappingOptions).field("requestHeaderMappings", &self.requestHeaderMappings).field("requestHeaderMappingCount", &self.requestHeaderMappingCount).field("responseHeaderMappings", &self.responseHeaderMappings).field("responseHeaderMappingCount", &self.responseHeaderMappingCount).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_HTTP_MESSAGE_MAPPING {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_HTTP_MESSAGE_MAPPING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_HTTP_MESSAGE_MAPPING>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_HTTP_MESSAGE_MAPPING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_HTTP_MESSAGE_MAPPING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HTTP_POLICY_DESCRIPTION {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
}
impl ::core::marker::Copy for WS_HTTP_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_HTTP_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HTTP_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_POLICY_DESCRIPTION").field("channelProperties", &self.channelProperties).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_HTTP_POLICY_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_HTTP_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_HTTP_POLICY_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_HTTP_POLICY_DESCRIPTION {}
impl ::core::default::Default for WS_HTTP_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_HTTP_PROXY_SETTING_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_PROXY_SETTING_MODE_AUTO: WS_HTTP_PROXY_SETTING_MODE = WS_HTTP_PROXY_SETTING_MODE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_PROXY_SETTING_MODE_NONE: WS_HTTP_PROXY_SETTING_MODE = WS_HTTP_PROXY_SETTING_MODE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_PROXY_SETTING_MODE_CUSTOM: WS_HTTP_PROXY_SETTING_MODE = WS_HTTP_PROXY_SETTING_MODE(3i32);
impl ::core::marker::Copy for WS_HTTP_PROXY_SETTING_MODE {}
impl ::core::clone::Clone for WS_HTTP_PROXY_SETTING_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_HTTP_PROXY_SETTING_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_HTTP_PROXY_SETTING_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_HTTP_PROXY_SETTING_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_HTTP_PROXY_SETTING_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_HTTP_REDIRECT_CALLBACK = ::core::option::Option<unsafe extern "system" fn(state: *const ::core::ffi::c_void, originalurl: *const WS_STRING, newurl: *const WS_STRING) -> ::windows::core::HRESULT>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HTTP_REDIRECT_CALLBACK_CONTEXT {
    pub callback: WS_HTTP_REDIRECT_CALLBACK,
    pub state: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WS_HTTP_REDIRECT_CALLBACK_CONTEXT {}
impl ::core::clone::Clone for WS_HTTP_REDIRECT_CALLBACK_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HTTP_REDIRECT_CALLBACK_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_REDIRECT_CALLBACK_CONTEXT").field("callback", &self.callback.map(|f| f as usize)).field("state", &self.state).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_HTTP_REDIRECT_CALLBACK_CONTEXT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_HTTP_REDIRECT_CALLBACK_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_HTTP_REDIRECT_CALLBACK_CONTEXT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_HTTP_REDIRECT_CALLBACK_CONTEXT {}
impl ::core::default::Default for WS_HTTP_REDIRECT_CALLBACK_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_REQUEST_MAPPING_VERB: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_RESPONSE_MAPPING_STATUS_CODE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_RESPONSE_MAPPING_STATUS_TEXT: i32 = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HTTP_SSL_BINDING_TEMPLATE {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sslTransportSecurityBinding: WS_SSL_TRANSPORT_SECURITY_BINDING_TEMPLATE,
}
impl ::core::marker::Copy for WS_HTTP_SSL_BINDING_TEMPLATE {}
impl ::core::clone::Clone for WS_HTTP_SSL_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HTTP_SSL_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_SSL_BINDING_TEMPLATE").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sslTransportSecurityBinding", &self.sslTransportSecurityBinding).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_HTTP_SSL_BINDING_TEMPLATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_HTTP_SSL_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_HTTP_SSL_BINDING_TEMPLATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_HTTP_SSL_BINDING_TEMPLATE {}
impl ::core::default::Default for WS_HTTP_SSL_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HTTP_SSL_HEADER_AUTH_BINDING_TEMPLATE {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sslTransportSecurityBinding: WS_SSL_TRANSPORT_SECURITY_BINDING_TEMPLATE,
    pub httpHeaderAuthSecurityBinding: WS_HTTP_HEADER_AUTH_SECURITY_BINDING_TEMPLATE,
}
impl ::core::marker::Copy for WS_HTTP_SSL_HEADER_AUTH_BINDING_TEMPLATE {}
impl ::core::clone::Clone for WS_HTTP_SSL_HEADER_AUTH_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HTTP_SSL_HEADER_AUTH_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_SSL_HEADER_AUTH_BINDING_TEMPLATE").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sslTransportSecurityBinding", &self.sslTransportSecurityBinding).field("httpHeaderAuthSecurityBinding", &self.httpHeaderAuthSecurityBinding).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_HTTP_SSL_HEADER_AUTH_BINDING_TEMPLATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_HTTP_SSL_HEADER_AUTH_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_HTTP_SSL_HEADER_AUTH_BINDING_TEMPLATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_HTTP_SSL_HEADER_AUTH_BINDING_TEMPLATE {}
impl ::core::default::Default for WS_HTTP_SSL_HEADER_AUTH_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HTTP_SSL_HEADER_AUTH_POLICY_DESCRIPTION {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sslTransportSecurityBinding: WS_SSL_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION,
    pub httpHeaderAuthSecurityBinding: WS_HTTP_HEADER_AUTH_SECURITY_BINDING_POLICY_DESCRIPTION,
}
impl ::core::marker::Copy for WS_HTTP_SSL_HEADER_AUTH_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_HTTP_SSL_HEADER_AUTH_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HTTP_SSL_HEADER_AUTH_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_SSL_HEADER_AUTH_POLICY_DESCRIPTION").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sslTransportSecurityBinding", &self.sslTransportSecurityBinding).field("httpHeaderAuthSecurityBinding", &self.httpHeaderAuthSecurityBinding).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_HTTP_SSL_HEADER_AUTH_POLICY_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_HTTP_SSL_HEADER_AUTH_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_HTTP_SSL_HEADER_AUTH_POLICY_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_HTTP_SSL_HEADER_AUTH_POLICY_DESCRIPTION {}
impl ::core::default::Default for WS_HTTP_SSL_HEADER_AUTH_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HTTP_SSL_KERBEROS_APREQ_BINDING_TEMPLATE {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sslTransportSecurityBinding: WS_SSL_TRANSPORT_SECURITY_BINDING_TEMPLATE,
    pub kerberosApreqMessageSecurityBinding: WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_TEMPLATE,
}
impl ::core::marker::Copy for WS_HTTP_SSL_KERBEROS_APREQ_BINDING_TEMPLATE {}
impl ::core::clone::Clone for WS_HTTP_SSL_KERBEROS_APREQ_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HTTP_SSL_KERBEROS_APREQ_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_SSL_KERBEROS_APREQ_BINDING_TEMPLATE").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sslTransportSecurityBinding", &self.sslTransportSecurityBinding).field("kerberosApreqMessageSecurityBinding", &self.kerberosApreqMessageSecurityBinding).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_HTTP_SSL_KERBEROS_APREQ_BINDING_TEMPLATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_HTTP_SSL_KERBEROS_APREQ_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_HTTP_SSL_KERBEROS_APREQ_BINDING_TEMPLATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_HTTP_SSL_KERBEROS_APREQ_BINDING_TEMPLATE {}
impl ::core::default::Default for WS_HTTP_SSL_KERBEROS_APREQ_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HTTP_SSL_KERBEROS_APREQ_POLICY_DESCRIPTION {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sslTransportSecurityBinding: WS_SSL_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION,
    pub kerberosApreqMessageSecurityBinding: WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION,
}
impl ::core::marker::Copy for WS_HTTP_SSL_KERBEROS_APREQ_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_HTTP_SSL_KERBEROS_APREQ_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HTTP_SSL_KERBEROS_APREQ_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_SSL_KERBEROS_APREQ_POLICY_DESCRIPTION").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sslTransportSecurityBinding", &self.sslTransportSecurityBinding).field("kerberosApreqMessageSecurityBinding", &self.kerberosApreqMessageSecurityBinding).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_HTTP_SSL_KERBEROS_APREQ_POLICY_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_HTTP_SSL_KERBEROS_APREQ_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_HTTP_SSL_KERBEROS_APREQ_POLICY_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_HTTP_SSL_KERBEROS_APREQ_POLICY_DESCRIPTION {}
impl ::core::default::Default for WS_HTTP_SSL_KERBEROS_APREQ_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sslTransportSecurityBinding: WS_SSL_TRANSPORT_SECURITY_BINDING_TEMPLATE,
    pub kerberosApreqMessageSecurityBinding: WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_TEMPLATE,
    pub securityContextSecurityBinding: WS_SECURITY_CONTEXT_SECURITY_BINDING_TEMPLATE,
}
impl ::core::marker::Copy for WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE {}
impl ::core::clone::Clone for WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sslTransportSecurityBinding", &self.sslTransportSecurityBinding).field("kerberosApreqMessageSecurityBinding", &self.kerberosApreqMessageSecurityBinding).field("securityContextSecurityBinding", &self.securityContextSecurityBinding).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE {}
impl ::core::default::Default for WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sslTransportSecurityBinding: WS_SSL_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION,
    pub kerberosApreqMessageSecurityBinding: WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION,
    pub securityContextSecurityBinding: WS_SECURITY_CONTEXT_SECURITY_BINDING_POLICY_DESCRIPTION,
}
impl ::core::marker::Copy for WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sslTransportSecurityBinding", &self.sslTransportSecurityBinding).field("kerberosApreqMessageSecurityBinding", &self.kerberosApreqMessageSecurityBinding).field("securityContextSecurityBinding", &self.securityContextSecurityBinding).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION {}
impl ::core::default::Default for WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HTTP_SSL_POLICY_DESCRIPTION {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sslTransportSecurityBinding: WS_SSL_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION,
}
impl ::core::marker::Copy for WS_HTTP_SSL_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_HTTP_SSL_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HTTP_SSL_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_SSL_POLICY_DESCRIPTION").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sslTransportSecurityBinding", &self.sslTransportSecurityBinding).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_HTTP_SSL_POLICY_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_HTTP_SSL_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_HTTP_SSL_POLICY_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_HTTP_SSL_POLICY_DESCRIPTION {}
impl ::core::default::Default for WS_HTTP_SSL_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HTTP_SSL_USERNAME_BINDING_TEMPLATE {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sslTransportSecurityBinding: WS_SSL_TRANSPORT_SECURITY_BINDING_TEMPLATE,
    pub usernameMessageSecurityBinding: WS_USERNAME_MESSAGE_SECURITY_BINDING_TEMPLATE,
}
impl ::core::marker::Copy for WS_HTTP_SSL_USERNAME_BINDING_TEMPLATE {}
impl ::core::clone::Clone for WS_HTTP_SSL_USERNAME_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HTTP_SSL_USERNAME_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_SSL_USERNAME_BINDING_TEMPLATE").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sslTransportSecurityBinding", &self.sslTransportSecurityBinding).field("usernameMessageSecurityBinding", &self.usernameMessageSecurityBinding).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_HTTP_SSL_USERNAME_BINDING_TEMPLATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_HTTP_SSL_USERNAME_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_HTTP_SSL_USERNAME_BINDING_TEMPLATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_HTTP_SSL_USERNAME_BINDING_TEMPLATE {}
impl ::core::default::Default for WS_HTTP_SSL_USERNAME_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HTTP_SSL_USERNAME_POLICY_DESCRIPTION {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sslTransportSecurityBinding: WS_SSL_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION,
    pub usernameMessageSecurityBinding: WS_USERNAME_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION,
}
impl ::core::marker::Copy for WS_HTTP_SSL_USERNAME_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_HTTP_SSL_USERNAME_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HTTP_SSL_USERNAME_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_SSL_USERNAME_POLICY_DESCRIPTION").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sslTransportSecurityBinding", &self.sslTransportSecurityBinding).field("usernameMessageSecurityBinding", &self.usernameMessageSecurityBinding).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_HTTP_SSL_USERNAME_POLICY_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_HTTP_SSL_USERNAME_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_HTTP_SSL_USERNAME_POLICY_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_HTTP_SSL_USERNAME_POLICY_DESCRIPTION {}
impl ::core::default::Default for WS_HTTP_SSL_USERNAME_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sslTransportSecurityBinding: WS_SSL_TRANSPORT_SECURITY_BINDING_TEMPLATE,
    pub usernameMessageSecurityBinding: WS_USERNAME_MESSAGE_SECURITY_BINDING_TEMPLATE,
    pub securityContextSecurityBinding: WS_SECURITY_CONTEXT_SECURITY_BINDING_TEMPLATE,
}
impl ::core::marker::Copy for WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE {}
impl ::core::clone::Clone for WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sslTransportSecurityBinding", &self.sslTransportSecurityBinding).field("usernameMessageSecurityBinding", &self.usernameMessageSecurityBinding).field("securityContextSecurityBinding", &self.securityContextSecurityBinding).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE {}
impl ::core::default::Default for WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sslTransportSecurityBinding: WS_SSL_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION,
    pub usernameMessageSecurityBinding: WS_USERNAME_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION,
    pub securityContextSecurityBinding: WS_SECURITY_CONTEXT_SECURITY_BINDING_POLICY_DESCRIPTION,
}
impl ::core::marker::Copy for WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sslTransportSecurityBinding", &self.sslTransportSecurityBinding).field("usernameMessageSecurityBinding", &self.usernameMessageSecurityBinding).field("securityContextSecurityBinding", &self.securityContextSecurityBinding).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION {}
impl ::core::default::Default for WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HTTP_URL {
    pub url: WS_URL,
    pub host: WS_STRING,
    pub port: u16,
    pub portAsString: WS_STRING,
    pub path: WS_STRING,
    pub query: WS_STRING,
    pub fragment: WS_STRING,
}
impl ::core::marker::Copy for WS_HTTP_URL {}
impl ::core::clone::Clone for WS_HTTP_URL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HTTP_URL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_URL").field("url", &self.url).field("host", &self.host).field("port", &self.port).field("portAsString", &self.portAsString).field("path", &self.path).field("query", &self.query).field("fragment", &self.fragment).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_HTTP_URL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_HTTP_URL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_HTTP_URL>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_HTTP_URL {}
impl ::core::default::Default for WS_HTTP_URL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_INT16_DESCRIPTION {
    pub minValue: i16,
    pub maxValue: i16,
}
impl ::core::marker::Copy for WS_INT16_DESCRIPTION {}
impl ::core::clone::Clone for WS_INT16_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_INT16_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_INT16_DESCRIPTION").field("minValue", &self.minValue).field("maxValue", &self.maxValue).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_INT16_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_INT16_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_INT16_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_INT16_DESCRIPTION {}
impl ::core::default::Default for WS_INT16_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_INT32_DESCRIPTION {
    pub minValue: i32,
    pub maxValue: i32,
}
impl ::core::marker::Copy for WS_INT32_DESCRIPTION {}
impl ::core::clone::Clone for WS_INT32_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_INT32_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_INT32_DESCRIPTION").field("minValue", &self.minValue).field("maxValue", &self.maxValue).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_INT32_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_INT32_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_INT32_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_INT32_DESCRIPTION {}
impl ::core::default::Default for WS_INT32_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_INT64_DESCRIPTION {
    pub minValue: i64,
    pub maxValue: i64,
}
impl ::core::marker::Copy for WS_INT64_DESCRIPTION {}
impl ::core::clone::Clone for WS_INT64_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_INT64_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_INT64_DESCRIPTION").field("minValue", &self.minValue).field("maxValue", &self.maxValue).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_INT64_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_INT64_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_INT64_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_INT64_DESCRIPTION {}
impl ::core::default::Default for WS_INT64_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_INT8_DESCRIPTION {
    pub minValue: super::super::Foundation::CHAR,
    pub maxValue: super::super::Foundation::CHAR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_INT8_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_INT8_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_INT8_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_INT8_DESCRIPTION").field("minValue", &self.minValue).field("maxValue", &self.maxValue).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_INT8_DESCRIPTION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_INT8_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_INT8_DESCRIPTION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_INT8_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_INT8_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_IP_VERSION(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_IP_VERSION_4: WS_IP_VERSION = WS_IP_VERSION(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_IP_VERSION_6: WS_IP_VERSION = WS_IP_VERSION(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_IP_VERSION_AUTO: WS_IP_VERSION = WS_IP_VERSION(3i32);
impl ::core::marker::Copy for WS_IP_VERSION {}
impl ::core::clone::Clone for WS_IP_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_IP_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_IP_VERSION {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_IP_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_IP_VERSION").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    pub bindingConstraint: WS_SECURITY_BINDING_CONSTRAINT,
    pub bindingUsage: WS_MESSAGE_SECURITY_USAGE,
    pub claimConstraints: *mut WS_XML_STRING,
    pub claimConstraintCount: u32,
    pub requestSecurityTokenPropertyConstraints: *mut WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT,
    pub requestSecurityTokenPropertyConstraintCount: u32,
    pub out: WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT")
            .field("bindingConstraint", &self.bindingConstraint)
            .field("bindingUsage", &self.bindingUsage)
            .field("claimConstraints", &self.claimConstraints)
            .field("claimConstraintCount", &self.claimConstraintCount)
            .field("requestSecurityTokenPropertyConstraints", &self.requestSecurityTokenPropertyConstraints)
            .field("requestSecurityTokenPropertyConstraintCount", &self.requestSecurityTokenPropertyConstraintCount)
            .field("out", &self.out)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT_0 {
    pub issuerAddress: *mut WS_ENDPOINT_ADDRESS,
    pub requestSecurityTokenTemplate: *mut WS_XML_BUFFER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT_0").field("issuerAddress", &self.issuerAddress).field("requestSecurityTokenTemplate", &self.requestSecurityTokenTemplate).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type WS_IS_DEFAULT_VALUE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(descriptiondata: *const ::core::ffi::c_void, value: *const ::core::ffi::c_void, defaultvalue: *const ::core::ffi::c_void, valuesize: u32, isdefault: *mut super::super::Foundation::BOOL, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_ITEM_RANGE {
    pub minItemCount: u32,
    pub maxItemCount: u32,
}
impl ::core::marker::Copy for WS_ITEM_RANGE {}
impl ::core::clone::Clone for WS_ITEM_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_ITEM_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ITEM_RANGE").field("minItemCount", &self.minItemCount).field("maxItemCount", &self.maxItemCount).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_ITEM_RANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_ITEM_RANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_ITEM_RANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_ITEM_RANGE {}
impl ::core::default::Default for WS_ITEM_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING {
    pub binding: WS_SECURITY_BINDING,
    pub bindingUsage: WS_MESSAGE_SECURITY_USAGE,
    pub clientCredential: *mut WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL,
}
impl ::core::marker::Copy for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING {}
impl ::core::clone::Clone for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING").field("binding", &self.binding).field("bindingUsage", &self.bindingUsage).field("clientCredential", &self.clientCredential).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING {}
impl ::core::default::Default for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    pub bindingConstraint: WS_SECURITY_BINDING_CONSTRAINT,
    pub bindingUsage: WS_MESSAGE_SECURITY_USAGE,
}
impl ::core::marker::Copy for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_CONSTRAINT {}
impl ::core::clone::Clone for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_CONSTRAINT").field("bindingConstraint", &self.bindingConstraint).field("bindingUsage", &self.bindingUsage).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_CONSTRAINT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_CONSTRAINT {}
impl ::core::default::Default for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {
    pub securityBindingProperties: WS_SECURITY_BINDING_PROPERTIES,
    pub bindingUsage: WS_MESSAGE_SECURITY_USAGE,
}
impl ::core::marker::Copy for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION").field("securityBindingProperties", &self.securityBindingProperties).field("bindingUsage", &self.bindingUsage).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {}
impl ::core::default::Default for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_TEMPLATE {
    pub securityBindingProperties: WS_SECURITY_BINDING_PROPERTIES,
    pub clientCredential: *mut WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL,
}
impl ::core::marker::Copy for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_TEMPLATE {}
impl ::core::clone::Clone for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_TEMPLATE").field("securityBindingProperties", &self.securityBindingProperties).field("clientCredential", &self.clientCredential).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_TEMPLATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_TEMPLATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_TEMPLATE {}
impl ::core::default::Default for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WS_LISTENER(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_LISTENER_PROPERTIES {
    pub properties: *mut WS_LISTENER_PROPERTY,
    pub propertyCount: u32,
}
impl ::core::marker::Copy for WS_LISTENER_PROPERTIES {}
impl ::core::clone::Clone for WS_LISTENER_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_LISTENER_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_LISTENER_PROPERTIES").field("properties", &self.properties).field("propertyCount", &self.propertyCount).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_LISTENER_PROPERTIES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_LISTENER_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_LISTENER_PROPERTIES>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_LISTENER_PROPERTIES {}
impl ::core::default::Default for WS_LISTENER_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_LISTENER_PROPERTY {
    pub id: WS_LISTENER_PROPERTY_ID,
    pub value: *mut ::core::ffi::c_void,
    pub valueSize: u32,
}
impl ::core::marker::Copy for WS_LISTENER_PROPERTY {}
impl ::core::clone::Clone for WS_LISTENER_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_LISTENER_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_LISTENER_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_LISTENER_PROPERTY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_LISTENER_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_LISTENER_PROPERTY>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_LISTENER_PROPERTY {}
impl ::core::default::Default for WS_LISTENER_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_LISTENER_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LISTENER_PROPERTY_LISTEN_BACKLOG: WS_LISTENER_PROPERTY_ID = WS_LISTENER_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LISTENER_PROPERTY_IP_VERSION: WS_LISTENER_PROPERTY_ID = WS_LISTENER_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LISTENER_PROPERTY_STATE: WS_LISTENER_PROPERTY_ID = WS_LISTENER_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LISTENER_PROPERTY_ASYNC_CALLBACK_MODEL: WS_LISTENER_PROPERTY_ID = WS_LISTENER_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LISTENER_PROPERTY_CHANNEL_TYPE: WS_LISTENER_PROPERTY_ID = WS_LISTENER_PROPERTY_ID(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LISTENER_PROPERTY_CHANNEL_BINDING: WS_LISTENER_PROPERTY_ID = WS_LISTENER_PROPERTY_ID(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LISTENER_PROPERTY_CONNECT_TIMEOUT: WS_LISTENER_PROPERTY_ID = WS_LISTENER_PROPERTY_ID(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LISTENER_PROPERTY_IS_MULTICAST: WS_LISTENER_PROPERTY_ID = WS_LISTENER_PROPERTY_ID(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LISTENER_PROPERTY_MULTICAST_INTERFACES: WS_LISTENER_PROPERTY_ID = WS_LISTENER_PROPERTY_ID(8i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LISTENER_PROPERTY_MULTICAST_LOOPBACK: WS_LISTENER_PROPERTY_ID = WS_LISTENER_PROPERTY_ID(9i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LISTENER_PROPERTY_CLOSE_TIMEOUT: WS_LISTENER_PROPERTY_ID = WS_LISTENER_PROPERTY_ID(10i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LISTENER_PROPERTY_TO_HEADER_MATCHING_OPTIONS: WS_LISTENER_PROPERTY_ID = WS_LISTENER_PROPERTY_ID(11i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LISTENER_PROPERTY_TRANSPORT_URL_MATCHING_OPTIONS: WS_LISTENER_PROPERTY_ID = WS_LISTENER_PROPERTY_ID(12i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LISTENER_PROPERTY_CUSTOM_LISTENER_CALLBACKS: WS_LISTENER_PROPERTY_ID = WS_LISTENER_PROPERTY_ID(13i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LISTENER_PROPERTY_CUSTOM_LISTENER_PARAMETERS: WS_LISTENER_PROPERTY_ID = WS_LISTENER_PROPERTY_ID(14i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LISTENER_PROPERTY_CUSTOM_LISTENER_INSTANCE: WS_LISTENER_PROPERTY_ID = WS_LISTENER_PROPERTY_ID(15i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LISTENER_PROPERTY_DISALLOWED_USER_AGENT: WS_LISTENER_PROPERTY_ID = WS_LISTENER_PROPERTY_ID(16i32);
impl ::core::marker::Copy for WS_LISTENER_PROPERTY_ID {}
impl ::core::clone::Clone for WS_LISTENER_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_LISTENER_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_LISTENER_PROPERTY_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_LISTENER_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_LISTENER_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_LISTENER_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LISTENER_STATE_CREATED: WS_LISTENER_STATE = WS_LISTENER_STATE(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LISTENER_STATE_OPENING: WS_LISTENER_STATE = WS_LISTENER_STATE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LISTENER_STATE_OPEN: WS_LISTENER_STATE = WS_LISTENER_STATE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LISTENER_STATE_FAULTED: WS_LISTENER_STATE = WS_LISTENER_STATE(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LISTENER_STATE_CLOSING: WS_LISTENER_STATE = WS_LISTENER_STATE(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LISTENER_STATE_CLOSED: WS_LISTENER_STATE = WS_LISTENER_STATE(5i32);
impl ::core::marker::Copy for WS_LISTENER_STATE {}
impl ::core::clone::Clone for WS_LISTENER_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_LISTENER_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_LISTENER_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_LISTENER_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_LISTENER_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MATCH_URL_DNS_FULLY_QUALIFIED_HOST: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MATCH_URL_DNS_HOST: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MATCH_URL_EXACT_PATH: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MATCH_URL_HOST_ADDRESSES: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MATCH_URL_LOCAL_HOST: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MATCH_URL_NETBIOS_HOST: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MATCH_URL_NO_QUERY: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MATCH_URL_PORT: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MATCH_URL_PREFIX_PATH: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MATCH_URL_THIS_HOST: i32 = 31i32;
#[repr(C)]
pub struct WS_MESSAGE(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_MESSAGE_DESCRIPTION {
    pub action: *mut WS_XML_STRING,
    pub bodyElementDescription: *mut WS_ELEMENT_DESCRIPTION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_MESSAGE_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_MESSAGE_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_MESSAGE_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_MESSAGE_DESCRIPTION").field("action", &self.action).field("bodyElementDescription", &self.bodyElementDescription).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_MESSAGE_DESCRIPTION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_MESSAGE_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_MESSAGE_DESCRIPTION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_MESSAGE_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_MESSAGE_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_MESSAGE_DONE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(donecallbackstate: *const ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_MESSAGE_INITIALIZATION(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_BLANK_MESSAGE: WS_MESSAGE_INITIALIZATION = WS_MESSAGE_INITIALIZATION(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_DUPLICATE_MESSAGE: WS_MESSAGE_INITIALIZATION = WS_MESSAGE_INITIALIZATION(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_REQUEST_MESSAGE: WS_MESSAGE_INITIALIZATION = WS_MESSAGE_INITIALIZATION(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_REPLY_MESSAGE: WS_MESSAGE_INITIALIZATION = WS_MESSAGE_INITIALIZATION(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_FAULT_MESSAGE: WS_MESSAGE_INITIALIZATION = WS_MESSAGE_INITIALIZATION(4i32);
impl ::core::marker::Copy for WS_MESSAGE_INITIALIZATION {}
impl ::core::clone::Clone for WS_MESSAGE_INITIALIZATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_MESSAGE_INITIALIZATION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_MESSAGE_INITIALIZATION {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_MESSAGE_INITIALIZATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_MESSAGE_INITIALIZATION").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_MESSAGE_PROPERTIES {
    pub properties: *mut WS_MESSAGE_PROPERTY,
    pub propertyCount: u32,
}
impl ::core::marker::Copy for WS_MESSAGE_PROPERTIES {}
impl ::core::clone::Clone for WS_MESSAGE_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_MESSAGE_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_MESSAGE_PROPERTIES").field("properties", &self.properties).field("propertyCount", &self.propertyCount).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_MESSAGE_PROPERTIES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_MESSAGE_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_MESSAGE_PROPERTIES>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_MESSAGE_PROPERTIES {}
impl ::core::default::Default for WS_MESSAGE_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_MESSAGE_PROPERTY {
    pub id: WS_MESSAGE_PROPERTY_ID,
    pub value: *mut ::core::ffi::c_void,
    pub valueSize: u32,
}
impl ::core::marker::Copy for WS_MESSAGE_PROPERTY {}
impl ::core::clone::Clone for WS_MESSAGE_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_MESSAGE_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_MESSAGE_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_MESSAGE_PROPERTY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_MESSAGE_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_MESSAGE_PROPERTY>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_MESSAGE_PROPERTY {}
impl ::core::default::Default for WS_MESSAGE_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_MESSAGE_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_PROPERTY_STATE: WS_MESSAGE_PROPERTY_ID = WS_MESSAGE_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_PROPERTY_HEAP: WS_MESSAGE_PROPERTY_ID = WS_MESSAGE_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_PROPERTY_ENVELOPE_VERSION: WS_MESSAGE_PROPERTY_ID = WS_MESSAGE_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_PROPERTY_ADDRESSING_VERSION: WS_MESSAGE_PROPERTY_ID = WS_MESSAGE_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_PROPERTY_HEADER_BUFFER: WS_MESSAGE_PROPERTY_ID = WS_MESSAGE_PROPERTY_ID(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_PROPERTY_HEADER_POSITION: WS_MESSAGE_PROPERTY_ID = WS_MESSAGE_PROPERTY_ID(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_PROPERTY_BODY_READER: WS_MESSAGE_PROPERTY_ID = WS_MESSAGE_PROPERTY_ID(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_PROPERTY_BODY_WRITER: WS_MESSAGE_PROPERTY_ID = WS_MESSAGE_PROPERTY_ID(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_PROPERTY_IS_ADDRESSED: WS_MESSAGE_PROPERTY_ID = WS_MESSAGE_PROPERTY_ID(8i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_PROPERTY_HEAP_PROPERTIES: WS_MESSAGE_PROPERTY_ID = WS_MESSAGE_PROPERTY_ID(9i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_PROPERTY_XML_READER_PROPERTIES: WS_MESSAGE_PROPERTY_ID = WS_MESSAGE_PROPERTY_ID(10i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_PROPERTY_XML_WRITER_PROPERTIES: WS_MESSAGE_PROPERTY_ID = WS_MESSAGE_PROPERTY_ID(11i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_PROPERTY_IS_FAULT: WS_MESSAGE_PROPERTY_ID = WS_MESSAGE_PROPERTY_ID(12i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_PROPERTY_MAX_PROCESSED_HEADERS: WS_MESSAGE_PROPERTY_ID = WS_MESSAGE_PROPERTY_ID(13i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_PROPERTY_USERNAME: WS_MESSAGE_PROPERTY_ID = WS_MESSAGE_PROPERTY_ID(14i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_PROPERTY_ENCODED_CERT: WS_MESSAGE_PROPERTY_ID = WS_MESSAGE_PROPERTY_ID(15i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_PROPERTY_TRANSPORT_SECURITY_WINDOWS_TOKEN: WS_MESSAGE_PROPERTY_ID = WS_MESSAGE_PROPERTY_ID(16i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_PROPERTY_HTTP_HEADER_AUTH_WINDOWS_TOKEN: WS_MESSAGE_PROPERTY_ID = WS_MESSAGE_PROPERTY_ID(17i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_PROPERTY_MESSAGE_SECURITY_WINDOWS_TOKEN: WS_MESSAGE_PROPERTY_ID = WS_MESSAGE_PROPERTY_ID(18i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_PROPERTY_SAML_ASSERTION: WS_MESSAGE_PROPERTY_ID = WS_MESSAGE_PROPERTY_ID(19i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_PROPERTY_SECURITY_CONTEXT: WS_MESSAGE_PROPERTY_ID = WS_MESSAGE_PROPERTY_ID(20i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_PROPERTY_PROTECTION_LEVEL: WS_MESSAGE_PROPERTY_ID = WS_MESSAGE_PROPERTY_ID(21i32);
impl ::core::marker::Copy for WS_MESSAGE_PROPERTY_ID {}
impl ::core::clone::Clone for WS_MESSAGE_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_MESSAGE_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_MESSAGE_PROPERTY_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_MESSAGE_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_MESSAGE_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_MESSAGE_SECURITY_USAGE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SUPPORTING_MESSAGE_SECURITY_USAGE: WS_MESSAGE_SECURITY_USAGE = WS_MESSAGE_SECURITY_USAGE(1i32);
impl ::core::marker::Copy for WS_MESSAGE_SECURITY_USAGE {}
impl ::core::clone::Clone for WS_MESSAGE_SECURITY_USAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_MESSAGE_SECURITY_USAGE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_MESSAGE_SECURITY_USAGE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_MESSAGE_SECURITY_USAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_MESSAGE_SECURITY_USAGE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_MESSAGE_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_STATE_EMPTY: WS_MESSAGE_STATE = WS_MESSAGE_STATE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_STATE_INITIALIZED: WS_MESSAGE_STATE = WS_MESSAGE_STATE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_STATE_READING: WS_MESSAGE_STATE = WS_MESSAGE_STATE(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_STATE_WRITING: WS_MESSAGE_STATE = WS_MESSAGE_STATE(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_STATE_DONE: WS_MESSAGE_STATE = WS_MESSAGE_STATE(5i32);
impl ::core::marker::Copy for WS_MESSAGE_STATE {}
impl ::core::clone::Clone for WS_MESSAGE_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_MESSAGE_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_MESSAGE_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_MESSAGE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_MESSAGE_STATE").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct WS_METADATA(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_METADATA_ENDPOINT {
    pub endpointAddress: WS_ENDPOINT_ADDRESS,
    pub endpointPolicy: *mut WS_POLICY,
    pub portName: *mut WS_XML_STRING,
    pub serviceName: *mut WS_XML_STRING,
    pub serviceNs: *mut WS_XML_STRING,
    pub bindingName: *mut WS_XML_STRING,
    pub bindingNs: *mut WS_XML_STRING,
    pub portTypeName: *mut WS_XML_STRING,
    pub portTypeNs: *mut WS_XML_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_METADATA_ENDPOINT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_METADATA_ENDPOINT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_METADATA_ENDPOINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_METADATA_ENDPOINT").field("endpointAddress", &self.endpointAddress).field("endpointPolicy", &self.endpointPolicy).field("portName", &self.portName).field("serviceName", &self.serviceName).field("serviceNs", &self.serviceNs).field("bindingName", &self.bindingName).field("bindingNs", &self.bindingNs).field("portTypeName", &self.portTypeName).field("portTypeNs", &self.portTypeNs).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_METADATA_ENDPOINT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_METADATA_ENDPOINT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_METADATA_ENDPOINT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_METADATA_ENDPOINT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_METADATA_ENDPOINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_METADATA_ENDPOINTS {
    pub endpoints: *mut WS_METADATA_ENDPOINT,
    pub endpointCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_METADATA_ENDPOINTS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_METADATA_ENDPOINTS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_METADATA_ENDPOINTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_METADATA_ENDPOINTS").field("endpoints", &self.endpoints).field("endpointCount", &self.endpointCount).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_METADATA_ENDPOINTS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_METADATA_ENDPOINTS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_METADATA_ENDPOINTS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_METADATA_ENDPOINTS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_METADATA_ENDPOINTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_METADATA_EXCHANGE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_METADATA_EXCHANGE_TYPE_NONE: WS_METADATA_EXCHANGE_TYPE = WS_METADATA_EXCHANGE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_METADATA_EXCHANGE_TYPE_MEX: WS_METADATA_EXCHANGE_TYPE = WS_METADATA_EXCHANGE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_METADATA_EXCHANGE_TYPE_HTTP_GET: WS_METADATA_EXCHANGE_TYPE = WS_METADATA_EXCHANGE_TYPE(2i32);
impl ::core::marker::Copy for WS_METADATA_EXCHANGE_TYPE {}
impl ::core::clone::Clone for WS_METADATA_EXCHANGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_METADATA_EXCHANGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_METADATA_EXCHANGE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_METADATA_EXCHANGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_METADATA_EXCHANGE_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_METADATA_PROPERTY {
    pub id: WS_METADATA_PROPERTY_ID,
    pub value: *mut ::core::ffi::c_void,
    pub valueSize: u32,
}
impl ::core::marker::Copy for WS_METADATA_PROPERTY {}
impl ::core::clone::Clone for WS_METADATA_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_METADATA_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_METADATA_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_METADATA_PROPERTY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_METADATA_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_METADATA_PROPERTY>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_METADATA_PROPERTY {}
impl ::core::default::Default for WS_METADATA_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_METADATA_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_METADATA_PROPERTY_STATE: WS_METADATA_PROPERTY_ID = WS_METADATA_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_METADATA_PROPERTY_HEAP_PROPERTIES: WS_METADATA_PROPERTY_ID = WS_METADATA_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_METADATA_PROPERTY_POLICY_PROPERTIES: WS_METADATA_PROPERTY_ID = WS_METADATA_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_METADATA_PROPERTY_HEAP_REQUESTED_SIZE: WS_METADATA_PROPERTY_ID = WS_METADATA_PROPERTY_ID(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_METADATA_PROPERTY_MAX_DOCUMENTS: WS_METADATA_PROPERTY_ID = WS_METADATA_PROPERTY_ID(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_METADATA_PROPERTY_HOST_NAMES: WS_METADATA_PROPERTY_ID = WS_METADATA_PROPERTY_ID(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_METADATA_PROPERTY_VERIFY_HOST_NAMES: WS_METADATA_PROPERTY_ID = WS_METADATA_PROPERTY_ID(7i32);
impl ::core::marker::Copy for WS_METADATA_PROPERTY_ID {}
impl ::core::clone::Clone for WS_METADATA_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_METADATA_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_METADATA_PROPERTY_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_METADATA_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_METADATA_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_METADATA_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_METADATA_STATE_CREATED: WS_METADATA_STATE = WS_METADATA_STATE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_METADATA_STATE_RESOLVED: WS_METADATA_STATE = WS_METADATA_STATE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_METADATA_STATE_FAULTED: WS_METADATA_STATE = WS_METADATA_STATE(3i32);
impl ::core::marker::Copy for WS_METADATA_STATE {}
impl ::core::clone::Clone for WS_METADATA_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_METADATA_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_METADATA_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_METADATA_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_METADATA_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_MOVE_TO(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MOVE_TO_ROOT_ELEMENT: WS_MOVE_TO = WS_MOVE_TO(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MOVE_TO_NEXT_ELEMENT: WS_MOVE_TO = WS_MOVE_TO(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MOVE_TO_PREVIOUS_ELEMENT: WS_MOVE_TO = WS_MOVE_TO(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MOVE_TO_CHILD_ELEMENT: WS_MOVE_TO = WS_MOVE_TO(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MOVE_TO_END_ELEMENT: WS_MOVE_TO = WS_MOVE_TO(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MOVE_TO_PARENT_ELEMENT: WS_MOVE_TO = WS_MOVE_TO(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MOVE_TO_NEXT_NODE: WS_MOVE_TO = WS_MOVE_TO(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MOVE_TO_PREVIOUS_NODE: WS_MOVE_TO = WS_MOVE_TO(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MOVE_TO_FIRST_NODE: WS_MOVE_TO = WS_MOVE_TO(8i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MOVE_TO_BOF: WS_MOVE_TO = WS_MOVE_TO(9i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MOVE_TO_EOF: WS_MOVE_TO = WS_MOVE_TO(10i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MOVE_TO_CHILD_NODE: WS_MOVE_TO = WS_MOVE_TO(11i32);
impl ::core::marker::Copy for WS_MOVE_TO {}
impl ::core::clone::Clone for WS_MOVE_TO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_MOVE_TO {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_MOVE_TO {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_MOVE_TO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_MOVE_TO").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MUST_UNDERSTAND_HEADER_ATTRIBUTE: i32 = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_NAMEDPIPE_SSPI_TRANSPORT_SECURITY_BINDING {
    pub binding: WS_SECURITY_BINDING,
    pub clientCredential: *mut WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL,
}
impl ::core::marker::Copy for WS_NAMEDPIPE_SSPI_TRANSPORT_SECURITY_BINDING {}
impl ::core::clone::Clone for WS_NAMEDPIPE_SSPI_TRANSPORT_SECURITY_BINDING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_NAMEDPIPE_SSPI_TRANSPORT_SECURITY_BINDING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_NAMEDPIPE_SSPI_TRANSPORT_SECURITY_BINDING").field("binding", &self.binding).field("clientCredential", &self.clientCredential).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_NAMEDPIPE_SSPI_TRANSPORT_SECURITY_BINDING {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_NAMEDPIPE_SSPI_TRANSPORT_SECURITY_BINDING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_NAMEDPIPE_SSPI_TRANSPORT_SECURITY_BINDING>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_NAMEDPIPE_SSPI_TRANSPORT_SECURITY_BINDING {}
impl ::core::default::Default for WS_NAMEDPIPE_SSPI_TRANSPORT_SECURITY_BINDING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(feature = "Win32_Security_Cryptography")]
pub struct WS_NCRYPT_ASYMMETRIC_SECURITY_KEY_HANDLE {
    pub keyHandle: WS_SECURITY_KEY_HANDLE,
    pub asymmetricKey: super::super::Security::Cryptography::NCRYPT_KEY_HANDLE,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::marker::Copy for WS_NCRYPT_ASYMMETRIC_SECURITY_KEY_HANDLE {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::clone::Clone for WS_NCRYPT_ASYMMETRIC_SECURITY_KEY_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::fmt::Debug for WS_NCRYPT_ASYMMETRIC_SECURITY_KEY_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_NCRYPT_ASYMMETRIC_SECURITY_KEY_HANDLE").field("keyHandle", &self.keyHandle).field("asymmetricKey", &self.asymmetricKey).finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
unsafe impl ::windows::core::Abi for WS_NCRYPT_ASYMMETRIC_SECURITY_KEY_HANDLE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::PartialEq for WS_NCRYPT_ASYMMETRIC_SECURITY_KEY_HANDLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_NCRYPT_ASYMMETRIC_SECURITY_KEY_HANDLE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::Eq for WS_NCRYPT_ASYMMETRIC_SECURITY_KEY_HANDLE {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for WS_NCRYPT_ASYMMETRIC_SECURITY_KEY_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_NETPIPE_URL {
    pub url: WS_URL,
    pub host: WS_STRING,
    pub port: u16,
    pub portAsString: WS_STRING,
    pub path: WS_STRING,
    pub query: WS_STRING,
    pub fragment: WS_STRING,
}
impl ::core::marker::Copy for WS_NETPIPE_URL {}
impl ::core::clone::Clone for WS_NETPIPE_URL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_NETPIPE_URL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_NETPIPE_URL").field("url", &self.url).field("host", &self.host).field("port", &self.port).field("portAsString", &self.portAsString).field("path", &self.path).field("query", &self.query).field("fragment", &self.fragment).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_NETPIPE_URL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_NETPIPE_URL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_NETPIPE_URL>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_NETPIPE_URL {}
impl ::core::default::Default for WS_NETPIPE_URL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_NETTCP_URL {
    pub url: WS_URL,
    pub host: WS_STRING,
    pub port: u16,
    pub portAsString: WS_STRING,
    pub path: WS_STRING,
    pub query: WS_STRING,
    pub fragment: WS_STRING,
}
impl ::core::marker::Copy for WS_NETTCP_URL {}
impl ::core::clone::Clone for WS_NETTCP_URL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_NETTCP_URL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_NETTCP_URL").field("url", &self.url).field("host", &self.host).field("port", &self.port).field("portAsString", &self.portAsString).field("path", &self.path).field("query", &self.query).field("fragment", &self.fragment).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_NETTCP_URL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_NETTCP_URL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_NETTCP_URL>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_NETTCP_URL {}
impl ::core::default::Default for WS_NETTCP_URL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_OPAQUE_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    pub credential: WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL,
    pub opaqueAuthIdentity: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WS_OPAQUE_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {}
impl ::core::clone::Clone for WS_OPAQUE_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_OPAQUE_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_OPAQUE_WINDOWS_INTEGRATED_AUTH_CREDENTIAL").field("credential", &self.credential).field("opaqueAuthIdentity", &self.opaqueAuthIdentity).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_OPAQUE_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_OPAQUE_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_OPAQUE_WINDOWS_INTEGRATED_AUTH_CREDENTIAL>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_OPAQUE_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {}
impl ::core::default::Default for WS_OPAQUE_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_OPEN_CHANNEL_CALLBACK = ::core::option::Option<unsafe extern "system" fn(channelinstance: *const ::core::ffi::c_void, endpointaddress: *const WS_ENDPOINT_ADDRESS, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_OPEN_LISTENER_CALLBACK = ::core::option::Option<unsafe extern "system" fn(listenerinstance: *const ::core::ffi::c_void, url: *const WS_STRING, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_OPERATION_CANCEL_CALLBACK = ::core::option::Option<unsafe extern "system" fn(reason: WS_SERVICE_CANCEL_REASON, state: *const ::core::ffi::c_void)>;
#[repr(C)]
pub struct WS_OPERATION_CONTEXT(pub u8);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_OPERATION_CONTEXT_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_OPERATION_CONTEXT_PROPERTY_CHANNEL: WS_OPERATION_CONTEXT_PROPERTY_ID = WS_OPERATION_CONTEXT_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_OPERATION_CONTEXT_PROPERTY_CONTRACT_DESCRIPTION: WS_OPERATION_CONTEXT_PROPERTY_ID = WS_OPERATION_CONTEXT_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_OPERATION_CONTEXT_PROPERTY_HOST_USER_STATE: WS_OPERATION_CONTEXT_PROPERTY_ID = WS_OPERATION_CONTEXT_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_OPERATION_CONTEXT_PROPERTY_CHANNEL_USER_STATE: WS_OPERATION_CONTEXT_PROPERTY_ID = WS_OPERATION_CONTEXT_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_OPERATION_CONTEXT_PROPERTY_INPUT_MESSAGE: WS_OPERATION_CONTEXT_PROPERTY_ID = WS_OPERATION_CONTEXT_PROPERTY_ID(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_OPERATION_CONTEXT_PROPERTY_OUTPUT_MESSAGE: WS_OPERATION_CONTEXT_PROPERTY_ID = WS_OPERATION_CONTEXT_PROPERTY_ID(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_OPERATION_CONTEXT_PROPERTY_HEAP: WS_OPERATION_CONTEXT_PROPERTY_ID = WS_OPERATION_CONTEXT_PROPERTY_ID(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_OPERATION_CONTEXT_PROPERTY_LISTENER: WS_OPERATION_CONTEXT_PROPERTY_ID = WS_OPERATION_CONTEXT_PROPERTY_ID(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_OPERATION_CONTEXT_PROPERTY_ENDPOINT_ADDRESS: WS_OPERATION_CONTEXT_PROPERTY_ID = WS_OPERATION_CONTEXT_PROPERTY_ID(8i32);
impl ::core::marker::Copy for WS_OPERATION_CONTEXT_PROPERTY_ID {}
impl ::core::clone::Clone for WS_OPERATION_CONTEXT_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_OPERATION_CONTEXT_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_OPERATION_CONTEXT_PROPERTY_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_OPERATION_CONTEXT_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_OPERATION_CONTEXT_PROPERTY_ID").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_OPERATION_DESCRIPTION {
    pub versionInfo: u32,
    pub inputMessageDescription: *mut WS_MESSAGE_DESCRIPTION,
    pub outputMessageDescription: *mut WS_MESSAGE_DESCRIPTION,
    pub inputMessageOptions: u32,
    pub outputMessageOptions: u32,
    pub parameterCount: u16,
    pub parameterDescription: *mut WS_PARAMETER_DESCRIPTION,
    pub stubCallback: WS_SERVICE_STUB_CALLBACK,
    pub style: WS_OPERATION_STYLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_OPERATION_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_OPERATION_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_OPERATION_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_OPERATION_DESCRIPTION")
            .field("versionInfo", &self.versionInfo)
            .field("inputMessageDescription", &self.inputMessageDescription)
            .field("outputMessageDescription", &self.outputMessageDescription)
            .field("inputMessageOptions", &self.inputMessageOptions)
            .field("outputMessageOptions", &self.outputMessageOptions)
            .field("parameterCount", &self.parameterCount)
            .field("parameterDescription", &self.parameterDescription)
            .field("stubCallback", &self.stubCallback.map(|f| f as usize))
            .field("style", &self.style)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_OPERATION_DESCRIPTION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_OPERATION_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_OPERATION_DESCRIPTION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_OPERATION_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_OPERATION_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_OPERATION_FREE_STATE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(state: *const ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_OPERATION_STYLE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_NON_RPC_LITERAL_OPERATION: WS_OPERATION_STYLE = WS_OPERATION_STYLE(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_RPC_LITERAL_OPERATION: WS_OPERATION_STYLE = WS_OPERATION_STYLE(1i32);
impl ::core::marker::Copy for WS_OPERATION_STYLE {}
impl ::core::clone::Clone for WS_OPERATION_STYLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_OPERATION_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_OPERATION_STYLE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_OPERATION_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_OPERATION_STYLE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_PARAMETER_DESCRIPTION {
    pub parameterType: WS_PARAMETER_TYPE,
    pub inputMessageIndex: u16,
    pub outputMessageIndex: u16,
}
impl ::core::marker::Copy for WS_PARAMETER_DESCRIPTION {}
impl ::core::clone::Clone for WS_PARAMETER_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_PARAMETER_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_PARAMETER_DESCRIPTION").field("parameterType", &self.parameterType).field("inputMessageIndex", &self.inputMessageIndex).field("outputMessageIndex", &self.outputMessageIndex).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_PARAMETER_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_PARAMETER_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_PARAMETER_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_PARAMETER_DESCRIPTION {}
impl ::core::default::Default for WS_PARAMETER_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_PARAMETER_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_PARAMETER_TYPE_NORMAL: WS_PARAMETER_TYPE = WS_PARAMETER_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_PARAMETER_TYPE_ARRAY: WS_PARAMETER_TYPE = WS_PARAMETER_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_PARAMETER_TYPE_ARRAY_COUNT: WS_PARAMETER_TYPE = WS_PARAMETER_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_PARAMETER_TYPE_MESSAGES: WS_PARAMETER_TYPE = WS_PARAMETER_TYPE(3i32);
impl ::core::marker::Copy for WS_PARAMETER_TYPE {}
impl ::core::clone::Clone for WS_PARAMETER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_PARAMETER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_PARAMETER_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_PARAMETER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_PARAMETER_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct WS_POLICY(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_POLICY_CONSTRAINTS {
    pub channelBinding: WS_CHANNEL_BINDING,
    pub channelPropertyConstraints: *mut WS_CHANNEL_PROPERTY_CONSTRAINT,
    pub channelPropertyConstraintCount: u32,
    pub securityConstraints: *mut WS_SECURITY_CONSTRAINTS,
    pub policyExtensions: *mut *mut WS_POLICY_EXTENSION,
    pub policyExtensionCount: u32,
}
impl ::core::marker::Copy for WS_POLICY_CONSTRAINTS {}
impl ::core::clone::Clone for WS_POLICY_CONSTRAINTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_POLICY_CONSTRAINTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_POLICY_CONSTRAINTS").field("channelBinding", &self.channelBinding).field("channelPropertyConstraints", &self.channelPropertyConstraints).field("channelPropertyConstraintCount", &self.channelPropertyConstraintCount).field("securityConstraints", &self.securityConstraints).field("policyExtensions", &self.policyExtensions).field("policyExtensionCount", &self.policyExtensionCount).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_POLICY_CONSTRAINTS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_POLICY_CONSTRAINTS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_POLICY_CONSTRAINTS>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_POLICY_CONSTRAINTS {}
impl ::core::default::Default for WS_POLICY_CONSTRAINTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_POLICY_EXTENSION {
    pub r#type: WS_POLICY_EXTENSION_TYPE,
}
impl ::core::marker::Copy for WS_POLICY_EXTENSION {}
impl ::core::clone::Clone for WS_POLICY_EXTENSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_POLICY_EXTENSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_POLICY_EXTENSION").field("type", &self.r#type).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_POLICY_EXTENSION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_POLICY_EXTENSION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_POLICY_EXTENSION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_POLICY_EXTENSION {}
impl ::core::default::Default for WS_POLICY_EXTENSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_POLICY_EXTENSION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ENDPOINT_POLICY_EXTENSION_TYPE: WS_POLICY_EXTENSION_TYPE = WS_POLICY_EXTENSION_TYPE(1i32);
impl ::core::marker::Copy for WS_POLICY_EXTENSION_TYPE {}
impl ::core::clone::Clone for WS_POLICY_EXTENSION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_POLICY_EXTENSION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_POLICY_EXTENSION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_POLICY_EXTENSION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_POLICY_EXTENSION_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_POLICY_PROPERTIES {
    pub properties: *mut WS_POLICY_PROPERTY,
    pub propertyCount: u32,
}
impl ::core::marker::Copy for WS_POLICY_PROPERTIES {}
impl ::core::clone::Clone for WS_POLICY_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_POLICY_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_POLICY_PROPERTIES").field("properties", &self.properties).field("propertyCount", &self.propertyCount).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_POLICY_PROPERTIES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_POLICY_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_POLICY_PROPERTIES>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_POLICY_PROPERTIES {}
impl ::core::default::Default for WS_POLICY_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_POLICY_PROPERTY {
    pub id: WS_POLICY_PROPERTY_ID,
    pub value: *mut ::core::ffi::c_void,
    pub valueSize: u32,
}
impl ::core::marker::Copy for WS_POLICY_PROPERTY {}
impl ::core::clone::Clone for WS_POLICY_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_POLICY_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_POLICY_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_POLICY_PROPERTY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_POLICY_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_POLICY_PROPERTY>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_POLICY_PROPERTY {}
impl ::core::default::Default for WS_POLICY_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_POLICY_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_POLICY_PROPERTY_STATE: WS_POLICY_PROPERTY_ID = WS_POLICY_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_POLICY_PROPERTY_MAX_ALTERNATIVES: WS_POLICY_PROPERTY_ID = WS_POLICY_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_POLICY_PROPERTY_MAX_DEPTH: WS_POLICY_PROPERTY_ID = WS_POLICY_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_POLICY_PROPERTY_MAX_EXTENSIONS: WS_POLICY_PROPERTY_ID = WS_POLICY_PROPERTY_ID(4i32);
impl ::core::marker::Copy for WS_POLICY_PROPERTY_ID {}
impl ::core::clone::Clone for WS_POLICY_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_POLICY_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_POLICY_PROPERTY_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_POLICY_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_POLICY_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_POLICY_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_POLICY_STATE_CREATED: WS_POLICY_STATE = WS_POLICY_STATE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_POLICY_STATE_FAULTED: WS_POLICY_STATE = WS_POLICY_STATE(2i32);
impl ::core::marker::Copy for WS_POLICY_STATE {}
impl ::core::clone::Clone for WS_POLICY_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_POLICY_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_POLICY_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_POLICY_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_POLICY_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_PROTECTION_LEVEL(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_PROTECTION_LEVEL_NONE: WS_PROTECTION_LEVEL = WS_PROTECTION_LEVEL(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_PROTECTION_LEVEL_SIGN: WS_PROTECTION_LEVEL = WS_PROTECTION_LEVEL(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_PROTECTION_LEVEL_SIGN_AND_ENCRYPT: WS_PROTECTION_LEVEL = WS_PROTECTION_LEVEL(3i32);
impl ::core::marker::Copy for WS_PROTECTION_LEVEL {}
impl ::core::clone::Clone for WS_PROTECTION_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_PROTECTION_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_PROTECTION_LEVEL {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_PROTECTION_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_PROTECTION_LEVEL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_PROXY_MESSAGE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(message: *const WS_MESSAGE, heap: *const WS_HEAP, state: *const ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_PROXY_MESSAGE_CALLBACK_CONTEXT {
    pub callback: WS_PROXY_MESSAGE_CALLBACK,
    pub state: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WS_PROXY_MESSAGE_CALLBACK_CONTEXT {}
impl ::core::clone::Clone for WS_PROXY_MESSAGE_CALLBACK_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_PROXY_MESSAGE_CALLBACK_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_PROXY_MESSAGE_CALLBACK_CONTEXT").field("callback", &self.callback.map(|f| f as usize)).field("state", &self.state).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_PROXY_MESSAGE_CALLBACK_CONTEXT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_PROXY_MESSAGE_CALLBACK_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_PROXY_MESSAGE_CALLBACK_CONTEXT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_PROXY_MESSAGE_CALLBACK_CONTEXT {}
impl ::core::default::Default for WS_PROXY_MESSAGE_CALLBACK_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_PROXY_PROPERTY {
    pub id: WS_PROXY_PROPERTY_ID,
    pub value: *mut ::core::ffi::c_void,
    pub valueSize: u32,
}
impl ::core::marker::Copy for WS_PROXY_PROPERTY {}
impl ::core::clone::Clone for WS_PROXY_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_PROXY_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_PROXY_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_PROXY_PROPERTY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_PROXY_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_PROXY_PROPERTY>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_PROXY_PROPERTY {}
impl ::core::default::Default for WS_PROXY_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_PROXY_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_PROXY_PROPERTY_CALL_TIMEOUT: WS_PROXY_PROPERTY_ID = WS_PROXY_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_PROXY_PROPERTY_MESSAGE_PROPERTIES: WS_PROXY_PROPERTY_ID = WS_PROXY_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_PROXY_PROPERTY_MAX_CALL_POOL_SIZE: WS_PROXY_PROPERTY_ID = WS_PROXY_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_PROXY_PROPERTY_STATE: WS_PROXY_PROPERTY_ID = WS_PROXY_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_PROXY_PROPERTY_MAX_PENDING_CALLS: WS_PROXY_PROPERTY_ID = WS_PROXY_PROPERTY_ID(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_PROXY_PROPERTY_MAX_CLOSE_TIMEOUT: WS_PROXY_PROPERTY_ID = WS_PROXY_PROPERTY_ID(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_PROXY_FAULT_LANG_ID: WS_PROXY_PROPERTY_ID = WS_PROXY_PROPERTY_ID(6i32);
impl ::core::marker::Copy for WS_PROXY_PROPERTY_ID {}
impl ::core::clone::Clone for WS_PROXY_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_PROXY_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_PROXY_PROPERTY_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_PROXY_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_PROXY_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_PULL_BYTES_CALLBACK = ::core::option::Option<unsafe extern "system" fn(callbackstate: *const ::core::ffi::c_void, bytes: *mut ::core::ffi::c_void, maxsize: u32, actualsize: *mut u32, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_PUSH_BYTES_CALLBACK = ::core::option::Option<unsafe extern "system" fn(callbackstate: *const ::core::ffi::c_void, writecallback: WS_WRITE_CALLBACK, writecallbackstate: *const ::core::ffi::c_void, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_RAW_SYMMETRIC_SECURITY_KEY_HANDLE {
    pub keyHandle: WS_SECURITY_KEY_HANDLE,
    pub rawKeyBytes: WS_BYTES,
}
impl ::core::marker::Copy for WS_RAW_SYMMETRIC_SECURITY_KEY_HANDLE {}
impl ::core::clone::Clone for WS_RAW_SYMMETRIC_SECURITY_KEY_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_RAW_SYMMETRIC_SECURITY_KEY_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_RAW_SYMMETRIC_SECURITY_KEY_HANDLE").field("keyHandle", &self.keyHandle).field("rawKeyBytes", &self.rawKeyBytes).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_RAW_SYMMETRIC_SECURITY_KEY_HANDLE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_RAW_SYMMETRIC_SECURITY_KEY_HANDLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_RAW_SYMMETRIC_SECURITY_KEY_HANDLE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_RAW_SYMMETRIC_SECURITY_KEY_HANDLE {}
impl ::core::default::Default for WS_RAW_SYMMETRIC_SECURITY_KEY_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_READ_CALLBACK = ::core::option::Option<unsafe extern "system" fn(callbackstate: *const ::core::ffi::c_void, bytes: *mut ::core::ffi::c_void, maxsize: u32, actualsize: *mut u32, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_READ_MESSAGE_END_CALLBACK = ::core::option::Option<unsafe extern "system" fn(channelinstance: *const ::core::ffi::c_void, message: *const WS_MESSAGE, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_READ_MESSAGE_START_CALLBACK = ::core::option::Option<unsafe extern "system" fn(channelinstance: *const ::core::ffi::c_void, message: *const WS_MESSAGE, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_READ_OPTION(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_READ_REQUIRED_VALUE: WS_READ_OPTION = WS_READ_OPTION(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_READ_REQUIRED_POINTER: WS_READ_OPTION = WS_READ_OPTION(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_READ_OPTIONAL_POINTER: WS_READ_OPTION = WS_READ_OPTION(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_READ_NILLABLE_POINTER: WS_READ_OPTION = WS_READ_OPTION(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_READ_NILLABLE_VALUE: WS_READ_OPTION = WS_READ_OPTION(5i32);
impl ::core::marker::Copy for WS_READ_OPTION {}
impl ::core::clone::Clone for WS_READ_OPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_READ_OPTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_READ_OPTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_READ_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_READ_OPTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_READ_TYPE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(reader: *const WS_XML_READER, typemapping: WS_TYPE_MAPPING, descriptiondata: *const ::core::ffi::c_void, heap: *const WS_HEAP, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_RECEIVE_OPTION(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_RECEIVE_REQUIRED_MESSAGE: WS_RECEIVE_OPTION = WS_RECEIVE_OPTION(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_RECEIVE_OPTIONAL_MESSAGE: WS_RECEIVE_OPTION = WS_RECEIVE_OPTION(2i32);
impl ::core::marker::Copy for WS_RECEIVE_OPTION {}
impl ::core::clone::Clone for WS_RECEIVE_OPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_RECEIVE_OPTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_RECEIVE_OPTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_RECEIVE_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_RECEIVE_OPTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_RELAY_HEADER_ATTRIBUTE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_REPEATING_HEADER_OPTION(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_REPEATING_HEADER: WS_REPEATING_HEADER_OPTION = WS_REPEATING_HEADER_OPTION(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SINGLETON_HEADER: WS_REPEATING_HEADER_OPTION = WS_REPEATING_HEADER_OPTION(2i32);
impl ::core::marker::Copy for WS_REPEATING_HEADER_OPTION {}
impl ::core::clone::Clone for WS_REPEATING_HEADER_OPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_REPEATING_HEADER_OPTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_REPEATING_HEADER_OPTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_REPEATING_HEADER_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_REPEATING_HEADER_OPTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_REQUEST_SECURITY_TOKEN_ACTION(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_REQUEST_SECURITY_TOKEN_ACTION_ISSUE: WS_REQUEST_SECURITY_TOKEN_ACTION = WS_REQUEST_SECURITY_TOKEN_ACTION(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_REQUEST_SECURITY_TOKEN_ACTION_NEW_CONTEXT: WS_REQUEST_SECURITY_TOKEN_ACTION = WS_REQUEST_SECURITY_TOKEN_ACTION(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_REQUEST_SECURITY_TOKEN_ACTION_RENEW_CONTEXT: WS_REQUEST_SECURITY_TOKEN_ACTION = WS_REQUEST_SECURITY_TOKEN_ACTION(3i32);
impl ::core::marker::Copy for WS_REQUEST_SECURITY_TOKEN_ACTION {}
impl ::core::clone::Clone for WS_REQUEST_SECURITY_TOKEN_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_REQUEST_SECURITY_TOKEN_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_REQUEST_SECURITY_TOKEN_ACTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_REQUEST_SECURITY_TOKEN_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_REQUEST_SECURITY_TOKEN_ACTION").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_REQUEST_SECURITY_TOKEN_PROPERTY {
    pub id: WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID,
    pub value: *mut ::core::ffi::c_void,
    pub valueSize: u32,
}
impl ::core::marker::Copy for WS_REQUEST_SECURITY_TOKEN_PROPERTY {}
impl ::core::clone::Clone for WS_REQUEST_SECURITY_TOKEN_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_REQUEST_SECURITY_TOKEN_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_REQUEST_SECURITY_TOKEN_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_REQUEST_SECURITY_TOKEN_PROPERTY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_REQUEST_SECURITY_TOKEN_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_REQUEST_SECURITY_TOKEN_PROPERTY>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_REQUEST_SECURITY_TOKEN_PROPERTY {}
impl ::core::default::Default for WS_REQUEST_SECURITY_TOKEN_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT {
    pub id: WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID,
    pub allowedValues: *mut ::core::ffi::c_void,
    pub allowedValuesSize: u32,
    pub out: WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT_0,
}
impl ::core::marker::Copy for WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT {}
impl ::core::clone::Clone for WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT").field("id", &self.id).field("allowedValues", &self.allowedValues).field("allowedValuesSize", &self.allowedValuesSize).field("out", &self.out).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT {}
impl ::core::default::Default for WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT_0 {
    pub requestSecurityTokenProperty: WS_REQUEST_SECURITY_TOKEN_PROPERTY,
}
impl ::core::marker::Copy for WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT_0 {}
impl ::core::clone::Clone for WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT_0").field("requestSecurityTokenProperty", &self.requestSecurityTokenProperty).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT_0 {}
impl ::core::default::Default for WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_REQUEST_SECURITY_TOKEN_PROPERTY_APPLIES_TO: WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID = WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_REQUEST_SECURITY_TOKEN_PROPERTY_TRUST_VERSION: WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID = WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_REQUEST_SECURITY_TOKEN_PROPERTY_SECURE_CONVERSATION_VERSION: WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID = WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_REQUEST_SECURITY_TOKEN_PROPERTY_ISSUED_TOKEN_TYPE: WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID = WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_REQUEST_SECURITY_TOKEN_PROPERTY_REQUEST_ACTION: WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID = WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_REQUEST_SECURITY_TOKEN_PROPERTY_EXISTING_TOKEN: WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID = WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_REQUEST_SECURITY_TOKEN_PROPERTY_ISSUED_TOKEN_KEY_TYPE: WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID = WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_REQUEST_SECURITY_TOKEN_PROPERTY_ISSUED_TOKEN_KEY_SIZE: WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID = WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID(8i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_REQUEST_SECURITY_TOKEN_PROPERTY_ISSUED_TOKEN_KEY_ENTROPY: WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID = WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID(9i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_REQUEST_SECURITY_TOKEN_PROPERTY_LOCAL_REQUEST_PARAMETERS: WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID = WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID(10i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_REQUEST_SECURITY_TOKEN_PROPERTY_SERVICE_REQUEST_PARAMETERS: WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID = WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID(11i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_REQUEST_SECURITY_TOKEN_PROPERTY_MESSAGE_PROPERTIES: WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID = WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID(12i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_REQUEST_SECURITY_TOKEN_PROPERTY_BEARER_KEY_TYPE_VERSION: WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID = WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID(13i32);
impl ::core::marker::Copy for WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID {}
impl ::core::clone::Clone for WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_RESET_CHANNEL_CALLBACK = ::core::option::Option<unsafe extern "system" fn(channelinstance: *const ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_RESET_LISTENER_CALLBACK = ::core::option::Option<unsafe extern "system" fn(listenerinstance: *const ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_RSA_ENDPOINT_IDENTITY {
    pub identity: WS_ENDPOINT_IDENTITY,
    pub modulus: WS_BYTES,
    pub exponent: WS_BYTES,
}
impl ::core::marker::Copy for WS_RSA_ENDPOINT_IDENTITY {}
impl ::core::clone::Clone for WS_RSA_ENDPOINT_IDENTITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_RSA_ENDPOINT_IDENTITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_RSA_ENDPOINT_IDENTITY").field("identity", &self.identity).field("modulus", &self.modulus).field("exponent", &self.exponent).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_RSA_ENDPOINT_IDENTITY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_RSA_ENDPOINT_IDENTITY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_RSA_ENDPOINT_IDENTITY>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_RSA_ENDPOINT_IDENTITY {}
impl ::core::default::Default for WS_RSA_ENDPOINT_IDENTITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SAML_AUTHENTICATOR {
    pub authenticatorType: WS_SAML_AUTHENTICATOR_TYPE,
}
impl ::core::marker::Copy for WS_SAML_AUTHENTICATOR {}
impl ::core::clone::Clone for WS_SAML_AUTHENTICATOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SAML_AUTHENTICATOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SAML_AUTHENTICATOR").field("authenticatorType", &self.authenticatorType).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_SAML_AUTHENTICATOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_SAML_AUTHENTICATOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_SAML_AUTHENTICATOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_SAML_AUTHENTICATOR {}
impl ::core::default::Default for WS_SAML_AUTHENTICATOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_SAML_AUTHENTICATOR_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CERT_SIGNED_SAML_AUTHENTICATOR_TYPE: WS_SAML_AUTHENTICATOR_TYPE = WS_SAML_AUTHENTICATOR_TYPE(1i32);
impl ::core::marker::Copy for WS_SAML_AUTHENTICATOR_TYPE {}
impl ::core::clone::Clone for WS_SAML_AUTHENTICATOR_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SAML_AUTHENTICATOR_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_SAML_AUTHENTICATOR_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_SAML_AUTHENTICATOR_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SAML_AUTHENTICATOR_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SAML_MESSAGE_SECURITY_BINDING {
    pub binding: WS_SECURITY_BINDING,
    pub bindingUsage: WS_MESSAGE_SECURITY_USAGE,
    pub authenticator: *mut WS_SAML_AUTHENTICATOR,
}
impl ::core::marker::Copy for WS_SAML_MESSAGE_SECURITY_BINDING {}
impl ::core::clone::Clone for WS_SAML_MESSAGE_SECURITY_BINDING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SAML_MESSAGE_SECURITY_BINDING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SAML_MESSAGE_SECURITY_BINDING").field("binding", &self.binding).field("bindingUsage", &self.bindingUsage).field("authenticator", &self.authenticator).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_SAML_MESSAGE_SECURITY_BINDING {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_SAML_MESSAGE_SECURITY_BINDING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_SAML_MESSAGE_SECURITY_BINDING>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_SAML_MESSAGE_SECURITY_BINDING {}
impl ::core::default::Default for WS_SAML_MESSAGE_SECURITY_BINDING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_SECURE_CONVERSATION_VERSION(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURE_CONVERSATION_VERSION_FEBRUARY_2005: WS_SECURE_CONVERSATION_VERSION = WS_SECURE_CONVERSATION_VERSION(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURE_CONVERSATION_VERSION_1_3: WS_SECURE_CONVERSATION_VERSION = WS_SECURE_CONVERSATION_VERSION(2i32);
impl ::core::marker::Copy for WS_SECURE_CONVERSATION_VERSION {}
impl ::core::clone::Clone for WS_SECURE_CONVERSATION_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SECURE_CONVERSATION_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_SECURE_CONVERSATION_VERSION {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_SECURE_CONVERSATION_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURE_CONVERSATION_VERSION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_SECURE_PROTOCOL(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURE_PROTOCOL_SSL2: WS_SECURE_PROTOCOL = WS_SECURE_PROTOCOL(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURE_PROTOCOL_SSL3: WS_SECURE_PROTOCOL = WS_SECURE_PROTOCOL(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURE_PROTOCOL_TLS1_0: WS_SECURE_PROTOCOL = WS_SECURE_PROTOCOL(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURE_PROTOCOL_TLS1_1: WS_SECURE_PROTOCOL = WS_SECURE_PROTOCOL(8i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURE_PROTOCOL_TLS1_2: WS_SECURE_PROTOCOL = WS_SECURE_PROTOCOL(16i32);
impl ::core::marker::Copy for WS_SECURE_PROTOCOL {}
impl ::core::clone::Clone for WS_SECURE_PROTOCOL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SECURE_PROTOCOL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_SECURE_PROTOCOL {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_SECURE_PROTOCOL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURE_PROTOCOL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_SECURITY_ALGORITHM_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_DEFAULT: WS_SECURITY_ALGORITHM_ID = WS_SECURITY_ALGORITHM_ID(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_CANONICALIZATION_EXCLUSIVE: WS_SECURITY_ALGORITHM_ID = WS_SECURITY_ALGORITHM_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_CANONICALIZATION_EXCLUSIVE_WITH_COMMENTS: WS_SECURITY_ALGORITHM_ID = WS_SECURITY_ALGORITHM_ID(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_DIGEST_SHA1: WS_SECURITY_ALGORITHM_ID = WS_SECURITY_ALGORITHM_ID(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_DIGEST_SHA_256: WS_SECURITY_ALGORITHM_ID = WS_SECURITY_ALGORITHM_ID(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_DIGEST_SHA_384: WS_SECURITY_ALGORITHM_ID = WS_SECURITY_ALGORITHM_ID(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_DIGEST_SHA_512: WS_SECURITY_ALGORITHM_ID = WS_SECURITY_ALGORITHM_ID(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_SYMMETRIC_SIGNATURE_HMAC_SHA1: WS_SECURITY_ALGORITHM_ID = WS_SECURITY_ALGORITHM_ID(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_SYMMETRIC_SIGNATURE_HMAC_SHA_256: WS_SECURITY_ALGORITHM_ID = WS_SECURITY_ALGORITHM_ID(8i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_SYMMETRIC_SIGNATURE_HMAC_SHA_384: WS_SECURITY_ALGORITHM_ID = WS_SECURITY_ALGORITHM_ID(9i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_SYMMETRIC_SIGNATURE_HMAC_SHA_512: WS_SECURITY_ALGORITHM_ID = WS_SECURITY_ALGORITHM_ID(10i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_ASYMMETRIC_SIGNATURE_RSA_SHA1: WS_SECURITY_ALGORITHM_ID = WS_SECURITY_ALGORITHM_ID(11i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_ASYMMETRIC_SIGNATURE_DSA_SHA1: WS_SECURITY_ALGORITHM_ID = WS_SECURITY_ALGORITHM_ID(12i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_ASYMMETRIC_SIGNATURE_RSA_SHA_256: WS_SECURITY_ALGORITHM_ID = WS_SECURITY_ALGORITHM_ID(13i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_ASYMMETRIC_SIGNATURE_RSA_SHA_384: WS_SECURITY_ALGORITHM_ID = WS_SECURITY_ALGORITHM_ID(14i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_ASYMMETRIC_SIGNATURE_RSA_SHA_512: WS_SECURITY_ALGORITHM_ID = WS_SECURITY_ALGORITHM_ID(15i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_ASYMMETRIC_KEYWRAP_RSA_1_5: WS_SECURITY_ALGORITHM_ID = WS_SECURITY_ALGORITHM_ID(16i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_ASYMMETRIC_KEYWRAP_RSA_OAEP: WS_SECURITY_ALGORITHM_ID = WS_SECURITY_ALGORITHM_ID(17i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_KEY_DERIVATION_P_SHA1: WS_SECURITY_ALGORITHM_ID = WS_SECURITY_ALGORITHM_ID(18i32);
impl ::core::marker::Copy for WS_SECURITY_ALGORITHM_ID {}
impl ::core::clone::Clone for WS_SECURITY_ALGORITHM_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SECURITY_ALGORITHM_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_SECURITY_ALGORITHM_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_SECURITY_ALGORITHM_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_ALGORITHM_ID").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SECURITY_ALGORITHM_PROPERTY {
    pub id: WS_SECURITY_ALGORITHM_PROPERTY_ID,
    pub value: *mut ::core::ffi::c_void,
    pub valueSize: u32,
}
impl ::core::marker::Copy for WS_SECURITY_ALGORITHM_PROPERTY {}
impl ::core::clone::Clone for WS_SECURITY_ALGORITHM_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SECURITY_ALGORITHM_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_ALGORITHM_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_SECURITY_ALGORITHM_PROPERTY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_SECURITY_ALGORITHM_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_SECURITY_ALGORITHM_PROPERTY>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_SECURITY_ALGORITHM_PROPERTY {}
impl ::core::default::Default for WS_SECURITY_ALGORITHM_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_SECURITY_ALGORITHM_PROPERTY_ID(pub i32);
impl ::core::marker::Copy for WS_SECURITY_ALGORITHM_PROPERTY_ID {}
impl ::core::clone::Clone for WS_SECURITY_ALGORITHM_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SECURITY_ALGORITHM_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_SECURITY_ALGORITHM_PROPERTY_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_SECURITY_ALGORITHM_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_ALGORITHM_PROPERTY_ID").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SECURITY_ALGORITHM_SUITE {
    pub canonicalizationAlgorithm: WS_SECURITY_ALGORITHM_ID,
    pub digestAlgorithm: WS_SECURITY_ALGORITHM_ID,
    pub symmetricSignatureAlgorithm: WS_SECURITY_ALGORITHM_ID,
    pub asymmetricSignatureAlgorithm: WS_SECURITY_ALGORITHM_ID,
    pub encryptionAlgorithm: WS_SECURITY_ALGORITHM_ID,
    pub keyDerivationAlgorithm: WS_SECURITY_ALGORITHM_ID,
    pub symmetricKeyWrapAlgorithm: WS_SECURITY_ALGORITHM_ID,
    pub asymmetricKeyWrapAlgorithm: WS_SECURITY_ALGORITHM_ID,
    pub minSymmetricKeyLength: u32,
    pub maxSymmetricKeyLength: u32,
    pub minAsymmetricKeyLength: u32,
    pub maxAsymmetricKeyLength: u32,
    pub properties: *mut WS_SECURITY_ALGORITHM_PROPERTY,
    pub propertyCount: u32,
}
impl ::core::marker::Copy for WS_SECURITY_ALGORITHM_SUITE {}
impl ::core::clone::Clone for WS_SECURITY_ALGORITHM_SUITE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SECURITY_ALGORITHM_SUITE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_ALGORITHM_SUITE")
            .field("canonicalizationAlgorithm", &self.canonicalizationAlgorithm)
            .field("digestAlgorithm", &self.digestAlgorithm)
            .field("symmetricSignatureAlgorithm", &self.symmetricSignatureAlgorithm)
            .field("asymmetricSignatureAlgorithm", &self.asymmetricSignatureAlgorithm)
            .field("encryptionAlgorithm", &self.encryptionAlgorithm)
            .field("keyDerivationAlgorithm", &self.keyDerivationAlgorithm)
            .field("symmetricKeyWrapAlgorithm", &self.symmetricKeyWrapAlgorithm)
            .field("asymmetricKeyWrapAlgorithm", &self.asymmetricKeyWrapAlgorithm)
            .field("minSymmetricKeyLength", &self.minSymmetricKeyLength)
            .field("maxSymmetricKeyLength", &self.maxSymmetricKeyLength)
            .field("minAsymmetricKeyLength", &self.minAsymmetricKeyLength)
            .field("maxAsymmetricKeyLength", &self.maxAsymmetricKeyLength)
            .field("properties", &self.properties)
            .field("propertyCount", &self.propertyCount)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for WS_SECURITY_ALGORITHM_SUITE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_SECURITY_ALGORITHM_SUITE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_SECURITY_ALGORITHM_SUITE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_SECURITY_ALGORITHM_SUITE {}
impl ::core::default::Default for WS_SECURITY_ALGORITHM_SUITE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_SECURITY_ALGORITHM_SUITE_NAME(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_SUITE_NAME_BASIC256: WS_SECURITY_ALGORITHM_SUITE_NAME = WS_SECURITY_ALGORITHM_SUITE_NAME(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_SUITE_NAME_BASIC192: WS_SECURITY_ALGORITHM_SUITE_NAME = WS_SECURITY_ALGORITHM_SUITE_NAME(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_SUITE_NAME_BASIC128: WS_SECURITY_ALGORITHM_SUITE_NAME = WS_SECURITY_ALGORITHM_SUITE_NAME(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_SUITE_NAME_BASIC256_RSA15: WS_SECURITY_ALGORITHM_SUITE_NAME = WS_SECURITY_ALGORITHM_SUITE_NAME(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_SUITE_NAME_BASIC192_RSA15: WS_SECURITY_ALGORITHM_SUITE_NAME = WS_SECURITY_ALGORITHM_SUITE_NAME(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_SUITE_NAME_BASIC128_RSA15: WS_SECURITY_ALGORITHM_SUITE_NAME = WS_SECURITY_ALGORITHM_SUITE_NAME(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_SUITE_NAME_BASIC256_SHA256: WS_SECURITY_ALGORITHM_SUITE_NAME = WS_SECURITY_ALGORITHM_SUITE_NAME(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_SUITE_NAME_BASIC192_SHA256: WS_SECURITY_ALGORITHM_SUITE_NAME = WS_SECURITY_ALGORITHM_SUITE_NAME(8i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_SUITE_NAME_BASIC128_SHA256: WS_SECURITY_ALGORITHM_SUITE_NAME = WS_SECURITY_ALGORITHM_SUITE_NAME(9i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_SUITE_NAME_BASIC256_SHA256_RSA15: WS_SECURITY_ALGORITHM_SUITE_NAME = WS_SECURITY_ALGORITHM_SUITE_NAME(10i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_SUITE_NAME_BASIC192_SHA256_RSA15: WS_SECURITY_ALGORITHM_SUITE_NAME = WS_SECURITY_ALGORITHM_SUITE_NAME(11i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_SUITE_NAME_BASIC128_SHA256_RSA15: WS_SECURITY_ALGORITHM_SUITE_NAME = WS_SECURITY_ALGORITHM_SUITE_NAME(12i32);
impl ::core::marker::Copy for WS_SECURITY_ALGORITHM_SUITE_NAME {}
impl ::core::clone::Clone for WS_SECURITY_ALGORITHM_SUITE_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SECURITY_ALGORITHM_SUITE_NAME {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_SECURITY_ALGORITHM_SUITE_NAME {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_SECURITY_ALGORITHM_SUITE_NAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_ALGORITHM_SUITE_NAME").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_SECURITY_BEARER_KEY_TYPE_VERSION(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BEARER_KEY_TYPE_VERSION_1_3_ORIGINAL_SPECIFICATION: WS_SECURITY_BEARER_KEY_TYPE_VERSION = WS_SECURITY_BEARER_KEY_TYPE_VERSION(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BEARER_KEY_TYPE_VERSION_1_3_ORIGINAL_SCHEMA: WS_SECURITY_BEARER_KEY_TYPE_VERSION = WS_SECURITY_BEARER_KEY_TYPE_VERSION(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BEARER_KEY_TYPE_VERSION_1_3_ERRATA_01: WS_SECURITY_BEARER_KEY_TYPE_VERSION = WS_SECURITY_BEARER_KEY_TYPE_VERSION(3i32);
impl ::core::marker::Copy for WS_SECURITY_BEARER_KEY_TYPE_VERSION {}
impl ::core::clone::Clone for WS_SECURITY_BEARER_KEY_TYPE_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SECURITY_BEARER_KEY_TYPE_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_SECURITY_BEARER_KEY_TYPE_VERSION {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_SECURITY_BEARER_KEY_TYPE_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_BEARER_KEY_TYPE_VERSION").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SECURITY_BINDING {
    pub bindingType: WS_SECURITY_BINDING_TYPE,
    pub properties: *mut WS_SECURITY_BINDING_PROPERTY,
    pub propertyCount: u32,
}
impl ::core::marker::Copy for WS_SECURITY_BINDING {}
impl ::core::clone::Clone for WS_SECURITY_BINDING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SECURITY_BINDING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_BINDING").field("bindingType", &self.bindingType).field("properties", &self.properties).field("propertyCount", &self.propertyCount).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_SECURITY_BINDING {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_SECURITY_BINDING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_SECURITY_BINDING>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_SECURITY_BINDING {}
impl ::core::default::Default for WS_SECURITY_BINDING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SECURITY_BINDING_CONSTRAINT {
    pub r#type: WS_SECURITY_BINDING_CONSTRAINT_TYPE,
    pub propertyConstraints: *mut WS_SECURITY_BINDING_PROPERTY_CONSTRAINT,
    pub propertyConstraintCount: u32,
}
impl ::core::marker::Copy for WS_SECURITY_BINDING_CONSTRAINT {}
impl ::core::clone::Clone for WS_SECURITY_BINDING_CONSTRAINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SECURITY_BINDING_CONSTRAINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_BINDING_CONSTRAINT").field("type", &self.r#type).field("propertyConstraints", &self.propertyConstraints).field("propertyConstraintCount", &self.propertyConstraintCount).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_SECURITY_BINDING_CONSTRAINT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_SECURITY_BINDING_CONSTRAINT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_SECURITY_BINDING_CONSTRAINT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_SECURITY_BINDING_CONSTRAINT {}
impl ::core::default::Default for WS_SECURITY_BINDING_CONSTRAINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_SECURITY_BINDING_CONSTRAINT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT_TYPE: WS_SECURITY_BINDING_CONSTRAINT_TYPE = WS_SECURITY_BINDING_CONSTRAINT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_CONSTRAINT_TYPE: WS_SECURITY_BINDING_CONSTRAINT_TYPE = WS_SECURITY_BINDING_CONSTRAINT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_HEADER_AUTH_SECURITY_BINDING_CONSTRAINT_TYPE: WS_SECURITY_BINDING_CONSTRAINT_TYPE = WS_SECURITY_BINDING_CONSTRAINT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_USERNAME_MESSAGE_SECURITY_BINDING_CONSTRAINT_TYPE: WS_SECURITY_BINDING_CONSTRAINT_TYPE = WS_SECURITY_BINDING_CONSTRAINT_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_CONSTRAINT_TYPE: WS_SECURITY_BINDING_CONSTRAINT_TYPE = WS_SECURITY_BINDING_CONSTRAINT_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT_TYPE: WS_SECURITY_BINDING_CONSTRAINT_TYPE = WS_SECURITY_BINDING_CONSTRAINT_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CERT_MESSAGE_SECURITY_BINDING_CONSTRAINT_TYPE: WS_SECURITY_BINDING_CONSTRAINT_TYPE = WS_SECURITY_BINDING_CONSTRAINT_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_CONSTRAINT_TYPE: WS_SECURITY_BINDING_CONSTRAINT_TYPE = WS_SECURITY_BINDING_CONSTRAINT_TYPE(8i32);
impl ::core::marker::Copy for WS_SECURITY_BINDING_CONSTRAINT_TYPE {}
impl ::core::clone::Clone for WS_SECURITY_BINDING_CONSTRAINT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SECURITY_BINDING_CONSTRAINT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_SECURITY_BINDING_CONSTRAINT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_SECURITY_BINDING_CONSTRAINT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_BINDING_CONSTRAINT_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SECURITY_BINDING_PROPERTIES {
    pub properties: *mut WS_SECURITY_BINDING_PROPERTY,
    pub propertyCount: u32,
}
impl ::core::marker::Copy for WS_SECURITY_BINDING_PROPERTIES {}
impl ::core::clone::Clone for WS_SECURITY_BINDING_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SECURITY_BINDING_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_BINDING_PROPERTIES").field("properties", &self.properties).field("propertyCount", &self.propertyCount).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_SECURITY_BINDING_PROPERTIES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_SECURITY_BINDING_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_SECURITY_BINDING_PROPERTIES>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_SECURITY_BINDING_PROPERTIES {}
impl ::core::default::Default for WS_SECURITY_BINDING_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SECURITY_BINDING_PROPERTY {
    pub id: WS_SECURITY_BINDING_PROPERTY_ID,
    pub value: *mut ::core::ffi::c_void,
    pub valueSize: u32,
}
impl ::core::marker::Copy for WS_SECURITY_BINDING_PROPERTY {}
impl ::core::clone::Clone for WS_SECURITY_BINDING_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SECURITY_BINDING_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_BINDING_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_SECURITY_BINDING_PROPERTY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_SECURITY_BINDING_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_SECURITY_BINDING_PROPERTY>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_SECURITY_BINDING_PROPERTY {}
impl ::core::default::Default for WS_SECURITY_BINDING_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SECURITY_BINDING_PROPERTY_CONSTRAINT {
    pub id: WS_SECURITY_BINDING_PROPERTY_ID,
    pub allowedValues: *mut ::core::ffi::c_void,
    pub allowedValuesSize: u32,
    pub out: WS_SECURITY_BINDING_PROPERTY_CONSTRAINT_0,
}
impl ::core::marker::Copy for WS_SECURITY_BINDING_PROPERTY_CONSTRAINT {}
impl ::core::clone::Clone for WS_SECURITY_BINDING_PROPERTY_CONSTRAINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SECURITY_BINDING_PROPERTY_CONSTRAINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_BINDING_PROPERTY_CONSTRAINT").field("id", &self.id).field("allowedValues", &self.allowedValues).field("allowedValuesSize", &self.allowedValuesSize).field("out", &self.out).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_SECURITY_BINDING_PROPERTY_CONSTRAINT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_SECURITY_BINDING_PROPERTY_CONSTRAINT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_SECURITY_BINDING_PROPERTY_CONSTRAINT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_SECURITY_BINDING_PROPERTY_CONSTRAINT {}
impl ::core::default::Default for WS_SECURITY_BINDING_PROPERTY_CONSTRAINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SECURITY_BINDING_PROPERTY_CONSTRAINT_0 {
    pub securityBindingProperty: WS_SECURITY_BINDING_PROPERTY,
}
impl ::core::marker::Copy for WS_SECURITY_BINDING_PROPERTY_CONSTRAINT_0 {}
impl ::core::clone::Clone for WS_SECURITY_BINDING_PROPERTY_CONSTRAINT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SECURITY_BINDING_PROPERTY_CONSTRAINT_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_BINDING_PROPERTY_CONSTRAINT_0").field("securityBindingProperty", &self.securityBindingProperty).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_SECURITY_BINDING_PROPERTY_CONSTRAINT_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_SECURITY_BINDING_PROPERTY_CONSTRAINT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_SECURITY_BINDING_PROPERTY_CONSTRAINT_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_SECURITY_BINDING_PROPERTY_CONSTRAINT_0 {}
impl ::core::default::Default for WS_SECURITY_BINDING_PROPERTY_CONSTRAINT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_SECURITY_BINDING_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BINDING_PROPERTY_REQUIRE_SSL_CLIENT_CERT: WS_SECURITY_BINDING_PROPERTY_ID = WS_SECURITY_BINDING_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BINDING_PROPERTY_WINDOWS_INTEGRATED_AUTH_PACKAGE: WS_SECURITY_BINDING_PROPERTY_ID = WS_SECURITY_BINDING_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BINDING_PROPERTY_REQUIRE_SERVER_AUTH: WS_SECURITY_BINDING_PROPERTY_ID = WS_SECURITY_BINDING_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BINDING_PROPERTY_ALLOW_ANONYMOUS_CLIENTS: WS_SECURITY_BINDING_PROPERTY_ID = WS_SECURITY_BINDING_PROPERTY_ID(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BINDING_PROPERTY_ALLOWED_IMPERSONATION_LEVEL: WS_SECURITY_BINDING_PROPERTY_ID = WS_SECURITY_BINDING_PROPERTY_ID(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BINDING_PROPERTY_HTTP_HEADER_AUTH_SCHEME: WS_SECURITY_BINDING_PROPERTY_ID = WS_SECURITY_BINDING_PROPERTY_ID(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BINDING_PROPERTY_HTTP_HEADER_AUTH_TARGET: WS_SECURITY_BINDING_PROPERTY_ID = WS_SECURITY_BINDING_PROPERTY_ID(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BINDING_PROPERTY_HTTP_HEADER_AUTH_BASIC_REALM: WS_SECURITY_BINDING_PROPERTY_ID = WS_SECURITY_BINDING_PROPERTY_ID(8i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BINDING_PROPERTY_HTTP_HEADER_AUTH_DIGEST_REALM: WS_SECURITY_BINDING_PROPERTY_ID = WS_SECURITY_BINDING_PROPERTY_ID(9i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BINDING_PROPERTY_HTTP_HEADER_AUTH_DIGEST_DOMAIN: WS_SECURITY_BINDING_PROPERTY_ID = WS_SECURITY_BINDING_PROPERTY_ID(10i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BINDING_PROPERTY_SECURITY_CONTEXT_KEY_SIZE: WS_SECURITY_BINDING_PROPERTY_ID = WS_SECURITY_BINDING_PROPERTY_ID(11i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BINDING_PROPERTY_SECURITY_CONTEXT_KEY_ENTROPY_MODE: WS_SECURITY_BINDING_PROPERTY_ID = WS_SECURITY_BINDING_PROPERTY_ID(12i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BINDING_PROPERTY_MESSAGE_PROPERTIES: WS_SECURITY_BINDING_PROPERTY_ID = WS_SECURITY_BINDING_PROPERTY_ID(13i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BINDING_PROPERTY_SECURITY_CONTEXT_MAX_PENDING_CONTEXTS: WS_SECURITY_BINDING_PROPERTY_ID = WS_SECURITY_BINDING_PROPERTY_ID(14i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BINDING_PROPERTY_SECURITY_CONTEXT_MAX_ACTIVE_CONTEXTS: WS_SECURITY_BINDING_PROPERTY_ID = WS_SECURITY_BINDING_PROPERTY_ID(15i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BINDING_PROPERTY_SECURE_CONVERSATION_VERSION: WS_SECURITY_BINDING_PROPERTY_ID = WS_SECURITY_BINDING_PROPERTY_ID(16i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BINDING_PROPERTY_SECURITY_CONTEXT_SUPPORT_RENEW: WS_SECURITY_BINDING_PROPERTY_ID = WS_SECURITY_BINDING_PROPERTY_ID(17i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BINDING_PROPERTY_SECURITY_CONTEXT_RENEWAL_INTERVAL: WS_SECURITY_BINDING_PROPERTY_ID = WS_SECURITY_BINDING_PROPERTY_ID(18i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BINDING_PROPERTY_SECURITY_CONTEXT_ROLLOVER_INTERVAL: WS_SECURITY_BINDING_PROPERTY_ID = WS_SECURITY_BINDING_PROPERTY_ID(19i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BINDING_PROPERTY_CERT_FAILURES_TO_IGNORE: WS_SECURITY_BINDING_PROPERTY_ID = WS_SECURITY_BINDING_PROPERTY_ID(20i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BINDING_PROPERTY_DISABLE_CERT_REVOCATION_CHECK: WS_SECURITY_BINDING_PROPERTY_ID = WS_SECURITY_BINDING_PROPERTY_ID(21i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BINDING_PROPERTY_DISALLOWED_SECURE_PROTOCOLS: WS_SECURITY_BINDING_PROPERTY_ID = WS_SECURITY_BINDING_PROPERTY_ID(22i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BINDING_PROPERTY_CERTIFICATE_VALIDATION_CALLBACK_CONTEXT: WS_SECURITY_BINDING_PROPERTY_ID = WS_SECURITY_BINDING_PROPERTY_ID(23i32);
impl ::core::marker::Copy for WS_SECURITY_BINDING_PROPERTY_ID {}
impl ::core::clone::Clone for WS_SECURITY_BINDING_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SECURITY_BINDING_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_SECURITY_BINDING_PROPERTY_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_SECURITY_BINDING_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_BINDING_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_SECURITY_BINDING_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SSL_TRANSPORT_SECURITY_BINDING_TYPE: WS_SECURITY_BINDING_TYPE = WS_SECURITY_BINDING_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_TYPE: WS_SECURITY_BINDING_TYPE = WS_SECURITY_BINDING_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_HEADER_AUTH_SECURITY_BINDING_TYPE: WS_SECURITY_BINDING_TYPE = WS_SECURITY_BINDING_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_USERNAME_MESSAGE_SECURITY_BINDING_TYPE: WS_SECURITY_BINDING_TYPE = WS_SECURITY_BINDING_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_TYPE: WS_SECURITY_BINDING_TYPE = WS_SECURITY_BINDING_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_TOKEN_MESSAGE_SECURITY_BINDING_TYPE: WS_SECURITY_BINDING_TYPE = WS_SECURITY_BINDING_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SAML_MESSAGE_SECURITY_BINDING_TYPE: WS_SECURITY_BINDING_TYPE = WS_SECURITY_BINDING_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_TYPE: WS_SECURITY_BINDING_TYPE = WS_SECURITY_BINDING_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_NAMEDPIPE_SSPI_TRANSPORT_SECURITY_BINDING_TYPE: WS_SECURITY_BINDING_TYPE = WS_SECURITY_BINDING_TYPE(9i32);
impl ::core::marker::Copy for WS_SECURITY_BINDING_TYPE {}
impl ::core::clone::Clone for WS_SECURITY_BINDING_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SECURITY_BINDING_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_SECURITY_BINDING_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_SECURITY_BINDING_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_BINDING_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SECURITY_CONSTRAINTS {
    pub securityPropertyConstraints: *mut WS_SECURITY_PROPERTY_CONSTRAINT,
    pub securityPropertyConstraintCount: u32,
    pub securityBindingConstraints: *mut *mut WS_SECURITY_BINDING_CONSTRAINT,
    pub securityBindingConstraintCount: u32,
}
impl ::core::marker::Copy for WS_SECURITY_CONSTRAINTS {}
impl ::core::clone::Clone for WS_SECURITY_CONSTRAINTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SECURITY_CONSTRAINTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_CONSTRAINTS").field("securityPropertyConstraints", &self.securityPropertyConstraints).field("securityPropertyConstraintCount", &self.securityPropertyConstraintCount).field("securityBindingConstraints", &self.securityBindingConstraints).field("securityBindingConstraintCount", &self.securityBindingConstraintCount).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_SECURITY_CONSTRAINTS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_SECURITY_CONSTRAINTS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_SECURITY_CONSTRAINTS>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_SECURITY_CONSTRAINTS {}
impl ::core::default::Default for WS_SECURITY_CONSTRAINTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WS_SECURITY_CONTEXT(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING {
    pub binding: WS_SECURITY_BINDING,
    pub bindingUsage: WS_MESSAGE_SECURITY_USAGE,
    pub bootstrapSecurityDescription: *mut WS_SECURITY_DESCRIPTION,
}
impl ::core::marker::Copy for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING {}
impl ::core::clone::Clone for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING").field("binding", &self.binding).field("bindingUsage", &self.bindingUsage).field("bootstrapSecurityDescription", &self.bootstrapSecurityDescription).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING {}
impl ::core::default::Default for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    pub bindingConstraint: WS_SECURITY_BINDING_CONSTRAINT,
    pub bindingUsage: WS_MESSAGE_SECURITY_USAGE,
    pub bootstrapSecurityConstraint: *mut WS_SECURITY_CONSTRAINTS,
}
impl ::core::marker::Copy for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_CONSTRAINT {}
impl ::core::clone::Clone for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_CONSTRAINT").field("bindingConstraint", &self.bindingConstraint).field("bindingUsage", &self.bindingUsage).field("bootstrapSecurityConstraint", &self.bootstrapSecurityConstraint).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_CONSTRAINT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_CONSTRAINT {}
impl ::core::default::Default for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {
    pub securityBindingProperties: WS_SECURITY_BINDING_PROPERTIES,
    pub bindingUsage: WS_MESSAGE_SECURITY_USAGE,
}
impl ::core::marker::Copy for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION").field("securityBindingProperties", &self.securityBindingProperties).field("bindingUsage", &self.bindingUsage).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {}
impl ::core::default::Default for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_TEMPLATE {
    pub securityBindingProperties: WS_SECURITY_BINDING_PROPERTIES,
}
impl ::core::marker::Copy for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_TEMPLATE {}
impl ::core::clone::Clone for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_TEMPLATE").field("securityBindingProperties", &self.securityBindingProperties).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_TEMPLATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_TEMPLATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_TEMPLATE {}
impl ::core::default::Default for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SECURITY_CONTEXT_PROPERTY {
    pub id: WS_SECURITY_CONTEXT_PROPERTY_ID,
    pub value: *mut ::core::ffi::c_void,
    pub valueSize: u32,
}
impl ::core::marker::Copy for WS_SECURITY_CONTEXT_PROPERTY {}
impl ::core::clone::Clone for WS_SECURITY_CONTEXT_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SECURITY_CONTEXT_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_CONTEXT_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_SECURITY_CONTEXT_PROPERTY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_SECURITY_CONTEXT_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_SECURITY_CONTEXT_PROPERTY>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_SECURITY_CONTEXT_PROPERTY {}
impl ::core::default::Default for WS_SECURITY_CONTEXT_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_SECURITY_CONTEXT_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_CONTEXT_PROPERTY_IDENTIFIER: WS_SECURITY_CONTEXT_PROPERTY_ID = WS_SECURITY_CONTEXT_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_CONTEXT_PROPERTY_USERNAME: WS_SECURITY_CONTEXT_PROPERTY_ID = WS_SECURITY_CONTEXT_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_CONTEXT_PROPERTY_MESSAGE_SECURITY_WINDOWS_TOKEN: WS_SECURITY_CONTEXT_PROPERTY_ID = WS_SECURITY_CONTEXT_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_CONTEXT_PROPERTY_SAML_ASSERTION: WS_SECURITY_CONTEXT_PROPERTY_ID = WS_SECURITY_CONTEXT_PROPERTY_ID(4i32);
impl ::core::marker::Copy for WS_SECURITY_CONTEXT_PROPERTY_ID {}
impl ::core::clone::Clone for WS_SECURITY_CONTEXT_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SECURITY_CONTEXT_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_SECURITY_CONTEXT_PROPERTY_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_SECURITY_CONTEXT_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_CONTEXT_PROPERTY_ID").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SECURITY_CONTEXT_SECURITY_BINDING_POLICY_DESCRIPTION {
    pub securityContextMessageSecurityBinding: WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION,
    pub securityProperties: WS_SECURITY_PROPERTIES,
}
impl ::core::marker::Copy for WS_SECURITY_CONTEXT_SECURITY_BINDING_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_SECURITY_CONTEXT_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SECURITY_CONTEXT_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_CONTEXT_SECURITY_BINDING_POLICY_DESCRIPTION").field("securityContextMessageSecurityBinding", &self.securityContextMessageSecurityBinding).field("securityProperties", &self.securityProperties).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_SECURITY_CONTEXT_SECURITY_BINDING_POLICY_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_SECURITY_CONTEXT_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_SECURITY_CONTEXT_SECURITY_BINDING_POLICY_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_SECURITY_CONTEXT_SECURITY_BINDING_POLICY_DESCRIPTION {}
impl ::core::default::Default for WS_SECURITY_CONTEXT_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SECURITY_CONTEXT_SECURITY_BINDING_TEMPLATE {
    pub securityContextMessageSecurityBinding: WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_TEMPLATE,
    pub securityProperties: WS_SECURITY_PROPERTIES,
}
impl ::core::marker::Copy for WS_SECURITY_CONTEXT_SECURITY_BINDING_TEMPLATE {}
impl ::core::clone::Clone for WS_SECURITY_CONTEXT_SECURITY_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SECURITY_CONTEXT_SECURITY_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_CONTEXT_SECURITY_BINDING_TEMPLATE").field("securityContextMessageSecurityBinding", &self.securityContextMessageSecurityBinding).field("securityProperties", &self.securityProperties).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_SECURITY_CONTEXT_SECURITY_BINDING_TEMPLATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_SECURITY_CONTEXT_SECURITY_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_SECURITY_CONTEXT_SECURITY_BINDING_TEMPLATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_SECURITY_CONTEXT_SECURITY_BINDING_TEMPLATE {}
impl ::core::default::Default for WS_SECURITY_CONTEXT_SECURITY_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SECURITY_DESCRIPTION {
    pub securityBindings: *mut *mut WS_SECURITY_BINDING,
    pub securityBindingCount: u32,
    pub properties: *mut WS_SECURITY_PROPERTY,
    pub propertyCount: u32,
}
impl ::core::marker::Copy for WS_SECURITY_DESCRIPTION {}
impl ::core::clone::Clone for WS_SECURITY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SECURITY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_DESCRIPTION").field("securityBindings", &self.securityBindings).field("securityBindingCount", &self.securityBindingCount).field("properties", &self.properties).field("propertyCount", &self.propertyCount).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_SECURITY_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_SECURITY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_SECURITY_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_SECURITY_DESCRIPTION {}
impl ::core::default::Default for WS_SECURITY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_SECURITY_HEADER_LAYOUT(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_HEADER_LAYOUT_STRICT: WS_SECURITY_HEADER_LAYOUT = WS_SECURITY_HEADER_LAYOUT(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_HEADER_LAYOUT_LAX: WS_SECURITY_HEADER_LAYOUT = WS_SECURITY_HEADER_LAYOUT(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_HEADER_LAYOUT_LAX_WITH_TIMESTAMP_FIRST: WS_SECURITY_HEADER_LAYOUT = WS_SECURITY_HEADER_LAYOUT(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_HEADER_LAYOUT_LAX_WITH_TIMESTAMP_LAST: WS_SECURITY_HEADER_LAYOUT = WS_SECURITY_HEADER_LAYOUT(4i32);
impl ::core::marker::Copy for WS_SECURITY_HEADER_LAYOUT {}
impl ::core::clone::Clone for WS_SECURITY_HEADER_LAYOUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SECURITY_HEADER_LAYOUT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_SECURITY_HEADER_LAYOUT {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_SECURITY_HEADER_LAYOUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_HEADER_LAYOUT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_SECURITY_HEADER_VERSION(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_HEADER_VERSION_1_0: WS_SECURITY_HEADER_VERSION = WS_SECURITY_HEADER_VERSION(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_HEADER_VERSION_1_1: WS_SECURITY_HEADER_VERSION = WS_SECURITY_HEADER_VERSION(2i32);
impl ::core::marker::Copy for WS_SECURITY_HEADER_VERSION {}
impl ::core::clone::Clone for WS_SECURITY_HEADER_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SECURITY_HEADER_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_SECURITY_HEADER_VERSION {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_SECURITY_HEADER_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_HEADER_VERSION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_SECURITY_KEY_ENTROPY_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_KEY_ENTROPY_MODE_CLIENT_ONLY: WS_SECURITY_KEY_ENTROPY_MODE = WS_SECURITY_KEY_ENTROPY_MODE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_KEY_ENTROPY_MODE_SERVER_ONLY: WS_SECURITY_KEY_ENTROPY_MODE = WS_SECURITY_KEY_ENTROPY_MODE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_KEY_ENTROPY_MODE_COMBINED: WS_SECURITY_KEY_ENTROPY_MODE = WS_SECURITY_KEY_ENTROPY_MODE(3i32);
impl ::core::marker::Copy for WS_SECURITY_KEY_ENTROPY_MODE {}
impl ::core::clone::Clone for WS_SECURITY_KEY_ENTROPY_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SECURITY_KEY_ENTROPY_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_SECURITY_KEY_ENTROPY_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_SECURITY_KEY_ENTROPY_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_KEY_ENTROPY_MODE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SECURITY_KEY_HANDLE {
    pub keyHandleType: WS_SECURITY_KEY_HANDLE_TYPE,
}
impl ::core::marker::Copy for WS_SECURITY_KEY_HANDLE {}
impl ::core::clone::Clone for WS_SECURITY_KEY_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SECURITY_KEY_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_KEY_HANDLE").field("keyHandleType", &self.keyHandleType).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_SECURITY_KEY_HANDLE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_SECURITY_KEY_HANDLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_SECURITY_KEY_HANDLE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_SECURITY_KEY_HANDLE {}
impl ::core::default::Default for WS_SECURITY_KEY_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_SECURITY_KEY_HANDLE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_RAW_SYMMETRIC_SECURITY_KEY_HANDLE_TYPE: WS_SECURITY_KEY_HANDLE_TYPE = WS_SECURITY_KEY_HANDLE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_NCRYPT_ASYMMETRIC_SECURITY_KEY_HANDLE_TYPE: WS_SECURITY_KEY_HANDLE_TYPE = WS_SECURITY_KEY_HANDLE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CAPI_ASYMMETRIC_SECURITY_KEY_HANDLE_TYPE: WS_SECURITY_KEY_HANDLE_TYPE = WS_SECURITY_KEY_HANDLE_TYPE(3i32);
impl ::core::marker::Copy for WS_SECURITY_KEY_HANDLE_TYPE {}
impl ::core::clone::Clone for WS_SECURITY_KEY_HANDLE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SECURITY_KEY_HANDLE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_SECURITY_KEY_HANDLE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_SECURITY_KEY_HANDLE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_KEY_HANDLE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_SECURITY_KEY_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_KEY_TYPE_NONE: WS_SECURITY_KEY_TYPE = WS_SECURITY_KEY_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_KEY_TYPE_SYMMETRIC: WS_SECURITY_KEY_TYPE = WS_SECURITY_KEY_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_KEY_TYPE_ASYMMETRIC: WS_SECURITY_KEY_TYPE = WS_SECURITY_KEY_TYPE(3i32);
impl ::core::marker::Copy for WS_SECURITY_KEY_TYPE {}
impl ::core::clone::Clone for WS_SECURITY_KEY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SECURITY_KEY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_SECURITY_KEY_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_SECURITY_KEY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_KEY_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SECURITY_PROPERTIES {
    pub properties: *mut WS_SECURITY_PROPERTY,
    pub propertyCount: u32,
}
impl ::core::marker::Copy for WS_SECURITY_PROPERTIES {}
impl ::core::clone::Clone for WS_SECURITY_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SECURITY_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_PROPERTIES").field("properties", &self.properties).field("propertyCount", &self.propertyCount).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_SECURITY_PROPERTIES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_SECURITY_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_SECURITY_PROPERTIES>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_SECURITY_PROPERTIES {}
impl ::core::default::Default for WS_SECURITY_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SECURITY_PROPERTY {
    pub id: WS_SECURITY_PROPERTY_ID,
    pub value: *mut ::core::ffi::c_void,
    pub valueSize: u32,
}
impl ::core::marker::Copy for WS_SECURITY_PROPERTY {}
impl ::core::clone::Clone for WS_SECURITY_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SECURITY_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_SECURITY_PROPERTY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_SECURITY_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_SECURITY_PROPERTY>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_SECURITY_PROPERTY {}
impl ::core::default::Default for WS_SECURITY_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SECURITY_PROPERTY_CONSTRAINT {
    pub id: WS_SECURITY_PROPERTY_ID,
    pub allowedValues: *mut ::core::ffi::c_void,
    pub allowedValuesSize: u32,
    pub out: WS_SECURITY_PROPERTY_CONSTRAINT_0,
}
impl ::core::marker::Copy for WS_SECURITY_PROPERTY_CONSTRAINT {}
impl ::core::clone::Clone for WS_SECURITY_PROPERTY_CONSTRAINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SECURITY_PROPERTY_CONSTRAINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_PROPERTY_CONSTRAINT").field("id", &self.id).field("allowedValues", &self.allowedValues).field("allowedValuesSize", &self.allowedValuesSize).field("out", &self.out).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_SECURITY_PROPERTY_CONSTRAINT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_SECURITY_PROPERTY_CONSTRAINT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_SECURITY_PROPERTY_CONSTRAINT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_SECURITY_PROPERTY_CONSTRAINT {}
impl ::core::default::Default for WS_SECURITY_PROPERTY_CONSTRAINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SECURITY_PROPERTY_CONSTRAINT_0 {
    pub securityProperty: WS_SECURITY_PROPERTY,
}
impl ::core::marker::Copy for WS_SECURITY_PROPERTY_CONSTRAINT_0 {}
impl ::core::clone::Clone for WS_SECURITY_PROPERTY_CONSTRAINT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SECURITY_PROPERTY_CONSTRAINT_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_PROPERTY_CONSTRAINT_0").field("securityProperty", &self.securityProperty).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_SECURITY_PROPERTY_CONSTRAINT_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_SECURITY_PROPERTY_CONSTRAINT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_SECURITY_PROPERTY_CONSTRAINT_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_SECURITY_PROPERTY_CONSTRAINT_0 {}
impl ::core::default::Default for WS_SECURITY_PROPERTY_CONSTRAINT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_SECURITY_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_PROPERTY_TRANSPORT_PROTECTION_LEVEL: WS_SECURITY_PROPERTY_ID = WS_SECURITY_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_PROPERTY_ALGORITHM_SUITE: WS_SECURITY_PROPERTY_ID = WS_SECURITY_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_PROPERTY_ALGORITHM_SUITE_NAME: WS_SECURITY_PROPERTY_ID = WS_SECURITY_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_PROPERTY_MAX_ALLOWED_LATENCY: WS_SECURITY_PROPERTY_ID = WS_SECURITY_PROPERTY_ID(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_PROPERTY_TIMESTAMP_VALIDITY_DURATION: WS_SECURITY_PROPERTY_ID = WS_SECURITY_PROPERTY_ID(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_PROPERTY_MAX_ALLOWED_CLOCK_SKEW: WS_SECURITY_PROPERTY_ID = WS_SECURITY_PROPERTY_ID(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_PROPERTY_TIMESTAMP_USAGE: WS_SECURITY_PROPERTY_ID = WS_SECURITY_PROPERTY_ID(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_PROPERTY_SECURITY_HEADER_LAYOUT: WS_SECURITY_PROPERTY_ID = WS_SECURITY_PROPERTY_ID(8i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_PROPERTY_SECURITY_HEADER_VERSION: WS_SECURITY_PROPERTY_ID = WS_SECURITY_PROPERTY_ID(9i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_PROPERTY_EXTENDED_PROTECTION_POLICY: WS_SECURITY_PROPERTY_ID = WS_SECURITY_PROPERTY_ID(10i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_PROPERTY_EXTENDED_PROTECTION_SCENARIO: WS_SECURITY_PROPERTY_ID = WS_SECURITY_PROPERTY_ID(11i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_PROPERTY_SERVICE_IDENTITIES: WS_SECURITY_PROPERTY_ID = WS_SECURITY_PROPERTY_ID(12i32);
impl ::core::marker::Copy for WS_SECURITY_PROPERTY_ID {}
impl ::core::clone::Clone for WS_SECURITY_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SECURITY_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_SECURITY_PROPERTY_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_SECURITY_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_SECURITY_TIMESTAMP_USAGE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_TIMESTAMP_USAGE_ALWAYS: WS_SECURITY_TIMESTAMP_USAGE = WS_SECURITY_TIMESTAMP_USAGE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_TIMESTAMP_USAGE_NEVER: WS_SECURITY_TIMESTAMP_USAGE = WS_SECURITY_TIMESTAMP_USAGE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_TIMESTAMP_USAGE_REQUESTS_ONLY: WS_SECURITY_TIMESTAMP_USAGE = WS_SECURITY_TIMESTAMP_USAGE(3i32);
impl ::core::marker::Copy for WS_SECURITY_TIMESTAMP_USAGE {}
impl ::core::clone::Clone for WS_SECURITY_TIMESTAMP_USAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SECURITY_TIMESTAMP_USAGE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_SECURITY_TIMESTAMP_USAGE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_SECURITY_TIMESTAMP_USAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_TIMESTAMP_USAGE").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct WS_SECURITY_TOKEN(pub u8);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_SECURITY_TOKEN_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_TOKEN_PROPERTY_KEY_TYPE: WS_SECURITY_TOKEN_PROPERTY_ID = WS_SECURITY_TOKEN_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_TOKEN_PROPERTY_VALID_FROM_TIME: WS_SECURITY_TOKEN_PROPERTY_ID = WS_SECURITY_TOKEN_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_TOKEN_PROPERTY_VALID_TILL_TIME: WS_SECURITY_TOKEN_PROPERTY_ID = WS_SECURITY_TOKEN_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_TOKEN_PROPERTY_SERIALIZED_XML: WS_SECURITY_TOKEN_PROPERTY_ID = WS_SECURITY_TOKEN_PROPERTY_ID(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_TOKEN_PROPERTY_ATTACHED_REFERENCE_XML: WS_SECURITY_TOKEN_PROPERTY_ID = WS_SECURITY_TOKEN_PROPERTY_ID(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_TOKEN_PROPERTY_UNATTACHED_REFERENCE_XML: WS_SECURITY_TOKEN_PROPERTY_ID = WS_SECURITY_TOKEN_PROPERTY_ID(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_TOKEN_PROPERTY_SYMMETRIC_KEY: WS_SECURITY_TOKEN_PROPERTY_ID = WS_SECURITY_TOKEN_PROPERTY_ID(7i32);
impl ::core::marker::Copy for WS_SECURITY_TOKEN_PROPERTY_ID {}
impl ::core::clone::Clone for WS_SECURITY_TOKEN_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SECURITY_TOKEN_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_SECURITY_TOKEN_PROPERTY_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_SECURITY_TOKEN_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_TOKEN_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_SECURITY_TOKEN_REFERENCE_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_TOKEN_REFERENCE_MODE_LOCAL_ID: WS_SECURITY_TOKEN_REFERENCE_MODE = WS_SECURITY_TOKEN_REFERENCE_MODE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_TOKEN_REFERENCE_MODE_XML_BUFFER: WS_SECURITY_TOKEN_REFERENCE_MODE = WS_SECURITY_TOKEN_REFERENCE_MODE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_TOKEN_REFERENCE_MODE_CERT_THUMBPRINT: WS_SECURITY_TOKEN_REFERENCE_MODE = WS_SECURITY_TOKEN_REFERENCE_MODE(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_TOKEN_REFERENCE_MODE_SECURITY_CONTEXT_ID: WS_SECURITY_TOKEN_REFERENCE_MODE = WS_SECURITY_TOKEN_REFERENCE_MODE(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_TOKEN_REFERENCE_MODE_SAML_ASSERTION_ID: WS_SECURITY_TOKEN_REFERENCE_MODE = WS_SECURITY_TOKEN_REFERENCE_MODE(5i32);
impl ::core::marker::Copy for WS_SECURITY_TOKEN_REFERENCE_MODE {}
impl ::core::clone::Clone for WS_SECURITY_TOKEN_REFERENCE_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SECURITY_TOKEN_REFERENCE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_SECURITY_TOKEN_REFERENCE_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_SECURITY_TOKEN_REFERENCE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_TOKEN_REFERENCE_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_SERVICE_ACCEPT_CHANNEL_CALLBACK = ::core::option::Option<unsafe extern "system" fn(context: *const WS_OPERATION_CONTEXT, channelstate: *mut *mut ::core::ffi::c_void, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_SERVICE_CANCEL_REASON(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_HOST_ABORT: WS_SERVICE_CANCEL_REASON = WS_SERVICE_CANCEL_REASON(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_CHANNEL_FAULTED: WS_SERVICE_CANCEL_REASON = WS_SERVICE_CANCEL_REASON(1i32);
impl ::core::marker::Copy for WS_SERVICE_CANCEL_REASON {}
impl ::core::clone::Clone for WS_SERVICE_CANCEL_REASON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SERVICE_CANCEL_REASON {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_SERVICE_CANCEL_REASON {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_SERVICE_CANCEL_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SERVICE_CANCEL_REASON").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_SERVICE_CLOSE_CHANNEL_CALLBACK = ::core::option::Option<unsafe extern "system" fn(context: *const WS_OPERATION_CONTEXT, asynccontext: *const WS_ASYNC_CONTEXT) -> ::windows::core::HRESULT>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_SERVICE_CONTRACT {
    pub contractDescription: *const WS_CONTRACT_DESCRIPTION,
    pub defaultMessageHandlerCallback: WS_SERVICE_MESSAGE_RECEIVE_CALLBACK,
    pub methodTable: *const ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_SERVICE_CONTRACT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_SERVICE_CONTRACT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_SERVICE_CONTRACT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SERVICE_CONTRACT").field("contractDescription", &self.contractDescription).field("defaultMessageHandlerCallback", &self.defaultMessageHandlerCallback.map(|f| f as usize)).field("methodTable", &self.methodTable).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_SERVICE_CONTRACT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_SERVICE_CONTRACT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_SERVICE_CONTRACT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_SERVICE_CONTRACT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_SERVICE_CONTRACT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_SERVICE_ENDPOINT {
    pub address: WS_ENDPOINT_ADDRESS,
    pub channelBinding: WS_CHANNEL_BINDING,
    pub channelType: WS_CHANNEL_TYPE,
    pub securityDescription: *const WS_SECURITY_DESCRIPTION,
    pub contract: *const WS_SERVICE_CONTRACT,
    pub authorizationCallback: WS_SERVICE_SECURITY_CALLBACK,
    pub properties: *const WS_SERVICE_ENDPOINT_PROPERTY,
    pub propertyCount: u32,
    pub channelProperties: WS_CHANNEL_PROPERTIES,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_SERVICE_ENDPOINT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_SERVICE_ENDPOINT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_SERVICE_ENDPOINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SERVICE_ENDPOINT").field("address", &self.address).field("channelBinding", &self.channelBinding).field("channelType", &self.channelType).field("securityDescription", &self.securityDescription).field("contract", &self.contract).field("authorizationCallback", &self.authorizationCallback.map(|f| f as usize)).field("properties", &self.properties).field("propertyCount", &self.propertyCount).field("channelProperties", &self.channelProperties).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_SERVICE_ENDPOINT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_SERVICE_ENDPOINT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_SERVICE_ENDPOINT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_SERVICE_ENDPOINT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_SERVICE_ENDPOINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_SERVICE_ENDPOINT_METADATA {
    pub portName: *mut WS_XML_STRING,
    pub bindingName: *mut WS_XML_STRING,
    pub bindingNs: *mut WS_XML_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_SERVICE_ENDPOINT_METADATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_SERVICE_ENDPOINT_METADATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_SERVICE_ENDPOINT_METADATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SERVICE_ENDPOINT_METADATA").field("portName", &self.portName).field("bindingName", &self.bindingName).field("bindingNs", &self.bindingNs).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_SERVICE_ENDPOINT_METADATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_SERVICE_ENDPOINT_METADATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_SERVICE_ENDPOINT_METADATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_SERVICE_ENDPOINT_METADATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_SERVICE_ENDPOINT_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SERVICE_ENDPOINT_PROPERTY {
    pub id: WS_SERVICE_ENDPOINT_PROPERTY_ID,
    pub value: *mut ::core::ffi::c_void,
    pub valueSize: u32,
}
impl ::core::marker::Copy for WS_SERVICE_ENDPOINT_PROPERTY {}
impl ::core::clone::Clone for WS_SERVICE_ENDPOINT_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SERVICE_ENDPOINT_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SERVICE_ENDPOINT_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_SERVICE_ENDPOINT_PROPERTY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_SERVICE_ENDPOINT_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_SERVICE_ENDPOINT_PROPERTY>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_SERVICE_ENDPOINT_PROPERTY {}
impl ::core::default::Default for WS_SERVICE_ENDPOINT_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_SERVICE_ENDPOINT_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_ENDPOINT_PROPERTY_ACCEPT_CHANNEL_CALLBACK: WS_SERVICE_ENDPOINT_PROPERTY_ID = WS_SERVICE_ENDPOINT_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_ENDPOINT_PROPERTY_CLOSE_CHANNEL_CALLBACK: WS_SERVICE_ENDPOINT_PROPERTY_ID = WS_SERVICE_ENDPOINT_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_ENDPOINT_PROPERTY_MAX_ACCEPTING_CHANNELS: WS_SERVICE_ENDPOINT_PROPERTY_ID = WS_SERVICE_ENDPOINT_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_ENDPOINT_PROPERTY_MAX_CONCURRENCY: WS_SERVICE_ENDPOINT_PROPERTY_ID = WS_SERVICE_ENDPOINT_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_ENDPOINT_PROPERTY_BODY_HEAP_MAX_SIZE: WS_SERVICE_ENDPOINT_PROPERTY_ID = WS_SERVICE_ENDPOINT_PROPERTY_ID(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_ENDPOINT_PROPERTY_BODY_HEAP_TRIM_SIZE: WS_SERVICE_ENDPOINT_PROPERTY_ID = WS_SERVICE_ENDPOINT_PROPERTY_ID(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_ENDPOINT_PROPERTY_MESSAGE_PROPERTIES: WS_SERVICE_ENDPOINT_PROPERTY_ID = WS_SERVICE_ENDPOINT_PROPERTY_ID(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_ENDPOINT_PROPERTY_MAX_CALL_POOL_SIZE: WS_SERVICE_ENDPOINT_PROPERTY_ID = WS_SERVICE_ENDPOINT_PROPERTY_ID(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_ENDPOINT_PROPERTY_MAX_CHANNEL_POOL_SIZE: WS_SERVICE_ENDPOINT_PROPERTY_ID = WS_SERVICE_ENDPOINT_PROPERTY_ID(8i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_ENDPOINT_PROPERTY_LISTENER_PROPERTIES: WS_SERVICE_ENDPOINT_PROPERTY_ID = WS_SERVICE_ENDPOINT_PROPERTY_ID(9i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_ENDPOINT_PROPERTY_CHECK_MUST_UNDERSTAND: WS_SERVICE_ENDPOINT_PROPERTY_ID = WS_SERVICE_ENDPOINT_PROPERTY_ID(10i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_ENDPOINT_PROPERTY_METADATA_EXCHANGE_TYPE: WS_SERVICE_ENDPOINT_PROPERTY_ID = WS_SERVICE_ENDPOINT_PROPERTY_ID(11i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_ENDPOINT_PROPERTY_METADATA: WS_SERVICE_ENDPOINT_PROPERTY_ID = WS_SERVICE_ENDPOINT_PROPERTY_ID(12i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_ENDPOINT_PROPERTY_METADATA_EXCHANGE_URL_SUFFIX: WS_SERVICE_ENDPOINT_PROPERTY_ID = WS_SERVICE_ENDPOINT_PROPERTY_ID(13i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_ENDPOINT_PROPERTY_MAX_CHANNELS: WS_SERVICE_ENDPOINT_PROPERTY_ID = WS_SERVICE_ENDPOINT_PROPERTY_ID(14i32);
impl ::core::marker::Copy for WS_SERVICE_ENDPOINT_PROPERTY_ID {}
impl ::core::clone::Clone for WS_SERVICE_ENDPOINT_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SERVICE_ENDPOINT_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_SERVICE_ENDPOINT_PROPERTY_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_SERVICE_ENDPOINT_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SERVICE_ENDPOINT_PROPERTY_ID").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct WS_SERVICE_HOST(pub u8);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_SERVICE_HOST_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_HOST_STATE_CREATED: WS_SERVICE_HOST_STATE = WS_SERVICE_HOST_STATE(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_HOST_STATE_OPENING: WS_SERVICE_HOST_STATE = WS_SERVICE_HOST_STATE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_HOST_STATE_OPEN: WS_SERVICE_HOST_STATE = WS_SERVICE_HOST_STATE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_HOST_STATE_CLOSING: WS_SERVICE_HOST_STATE = WS_SERVICE_HOST_STATE(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_HOST_STATE_CLOSED: WS_SERVICE_HOST_STATE = WS_SERVICE_HOST_STATE(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_HOST_STATE_FAULTED: WS_SERVICE_HOST_STATE = WS_SERVICE_HOST_STATE(5i32);
impl ::core::marker::Copy for WS_SERVICE_HOST_STATE {}
impl ::core::clone::Clone for WS_SERVICE_HOST_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SERVICE_HOST_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_SERVICE_HOST_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_SERVICE_HOST_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SERVICE_HOST_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_SERVICE_MESSAGE_RECEIVE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(context: *const WS_OPERATION_CONTEXT, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_SERVICE_METADATA {
    pub documentCount: u32,
    pub documents: *mut *mut WS_SERVICE_METADATA_DOCUMENT,
    pub serviceName: *mut WS_XML_STRING,
    pub serviceNs: *mut WS_XML_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_SERVICE_METADATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_SERVICE_METADATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_SERVICE_METADATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SERVICE_METADATA").field("documentCount", &self.documentCount).field("documents", &self.documents).field("serviceName", &self.serviceName).field("serviceNs", &self.serviceNs).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_SERVICE_METADATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_SERVICE_METADATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_SERVICE_METADATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_SERVICE_METADATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_SERVICE_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_SERVICE_METADATA_DOCUMENT {
    pub content: *mut WS_XML_STRING,
    pub name: *mut WS_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_SERVICE_METADATA_DOCUMENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_SERVICE_METADATA_DOCUMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_SERVICE_METADATA_DOCUMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SERVICE_METADATA_DOCUMENT").field("content", &self.content).field("name", &self.name).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_SERVICE_METADATA_DOCUMENT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_SERVICE_METADATA_DOCUMENT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_SERVICE_METADATA_DOCUMENT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_SERVICE_METADATA_DOCUMENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_SERVICE_METADATA_DOCUMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_OPERATION_MESSAGE_NILLABLE_ELEMENT: i32 = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SERVICE_PROPERTY {
    pub id: WS_SERVICE_PROPERTY_ID,
    pub value: *mut ::core::ffi::c_void,
    pub valueSize: u32,
}
impl ::core::marker::Copy for WS_SERVICE_PROPERTY {}
impl ::core::clone::Clone for WS_SERVICE_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SERVICE_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SERVICE_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_SERVICE_PROPERTY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_SERVICE_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_SERVICE_PROPERTY>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_SERVICE_PROPERTY {}
impl ::core::default::Default for WS_SERVICE_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SERVICE_PROPERTY_ACCEPT_CALLBACK {
    pub callback: WS_SERVICE_ACCEPT_CHANNEL_CALLBACK,
}
impl ::core::marker::Copy for WS_SERVICE_PROPERTY_ACCEPT_CALLBACK {}
impl ::core::clone::Clone for WS_SERVICE_PROPERTY_ACCEPT_CALLBACK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SERVICE_PROPERTY_ACCEPT_CALLBACK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SERVICE_PROPERTY_ACCEPT_CALLBACK").field("callback", &self.callback.map(|f| f as usize)).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_SERVICE_PROPERTY_ACCEPT_CALLBACK {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_SERVICE_PROPERTY_ACCEPT_CALLBACK {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_SERVICE_PROPERTY_ACCEPT_CALLBACK>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_SERVICE_PROPERTY_ACCEPT_CALLBACK {}
impl ::core::default::Default for WS_SERVICE_PROPERTY_ACCEPT_CALLBACK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SERVICE_PROPERTY_CLOSE_CALLBACK {
    pub callback: WS_SERVICE_CLOSE_CHANNEL_CALLBACK,
}
impl ::core::marker::Copy for WS_SERVICE_PROPERTY_CLOSE_CALLBACK {}
impl ::core::clone::Clone for WS_SERVICE_PROPERTY_CLOSE_CALLBACK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SERVICE_PROPERTY_CLOSE_CALLBACK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SERVICE_PROPERTY_CLOSE_CALLBACK").field("callback", &self.callback.map(|f| f as usize)).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_SERVICE_PROPERTY_CLOSE_CALLBACK {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_SERVICE_PROPERTY_CLOSE_CALLBACK {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_SERVICE_PROPERTY_CLOSE_CALLBACK>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_SERVICE_PROPERTY_CLOSE_CALLBACK {}
impl ::core::default::Default for WS_SERVICE_PROPERTY_CLOSE_CALLBACK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_SERVICE_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_PROPERTY_HOST_USER_STATE: WS_SERVICE_PROPERTY_ID = WS_SERVICE_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_PROPERTY_FAULT_DISCLOSURE: WS_SERVICE_PROPERTY_ID = WS_SERVICE_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_PROPERTY_FAULT_LANGID: WS_SERVICE_PROPERTY_ID = WS_SERVICE_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_PROPERTY_HOST_STATE: WS_SERVICE_PROPERTY_ID = WS_SERVICE_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_PROPERTY_METADATA: WS_SERVICE_PROPERTY_ID = WS_SERVICE_PROPERTY_ID(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_PROPERTY_CLOSE_TIMEOUT: WS_SERVICE_PROPERTY_ID = WS_SERVICE_PROPERTY_ID(5i32);
impl ::core::marker::Copy for WS_SERVICE_PROPERTY_ID {}
impl ::core::clone::Clone for WS_SERVICE_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SERVICE_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_SERVICE_PROPERTY_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_SERVICE_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SERVICE_PROPERTY_ID").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct WS_SERVICE_PROXY(pub u8);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_SERVICE_PROXY_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_PROXY_STATE_CREATED: WS_SERVICE_PROXY_STATE = WS_SERVICE_PROXY_STATE(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_PROXY_STATE_OPENING: WS_SERVICE_PROXY_STATE = WS_SERVICE_PROXY_STATE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_PROXY_STATE_OPEN: WS_SERVICE_PROXY_STATE = WS_SERVICE_PROXY_STATE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_PROXY_STATE_CLOSING: WS_SERVICE_PROXY_STATE = WS_SERVICE_PROXY_STATE(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_PROXY_STATE_CLOSED: WS_SERVICE_PROXY_STATE = WS_SERVICE_PROXY_STATE(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_PROXY_STATE_FAULTED: WS_SERVICE_PROXY_STATE = WS_SERVICE_PROXY_STATE(5i32);
impl ::core::marker::Copy for WS_SERVICE_PROXY_STATE {}
impl ::core::clone::Clone for WS_SERVICE_PROXY_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SERVICE_PROXY_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_SERVICE_PROXY_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_SERVICE_PROXY_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SERVICE_PROXY_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type WS_SERVICE_SECURITY_CALLBACK = ::core::option::Option<unsafe extern "system" fn(context: *const WS_OPERATION_CONTEXT, authorized: *mut super::super::Foundation::BOOL, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SERVICE_SECURITY_IDENTITIES {
    pub serviceIdentities: *mut WS_STRING,
    pub serviceIdentityCount: u32,
}
impl ::core::marker::Copy for WS_SERVICE_SECURITY_IDENTITIES {}
impl ::core::clone::Clone for WS_SERVICE_SECURITY_IDENTITIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SERVICE_SECURITY_IDENTITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SERVICE_SECURITY_IDENTITIES").field("serviceIdentities", &self.serviceIdentities).field("serviceIdentityCount", &self.serviceIdentityCount).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_SERVICE_SECURITY_IDENTITIES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_SERVICE_SECURITY_IDENTITIES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_SERVICE_SECURITY_IDENTITIES>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_SERVICE_SECURITY_IDENTITIES {}
impl ::core::default::Default for WS_SERVICE_SECURITY_IDENTITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_SERVICE_STUB_CALLBACK = ::core::option::Option<unsafe extern "system" fn(context: *const WS_OPERATION_CONTEXT, frame: *const ::core::ffi::c_void, callback: *const ::core::ffi::c_void, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_SET_CHANNEL_PROPERTY_CALLBACK = ::core::option::Option<unsafe extern "system" fn(channelinstance: *const ::core::ffi::c_void, id: WS_CHANNEL_PROPERTY_ID, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_SET_LISTENER_PROPERTY_CALLBACK = ::core::option::Option<unsafe extern "system" fn(listenerinstance: *const ::core::ffi::c_void, id: WS_LISTENER_PROPERTY_ID, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_SHUTDOWN_SESSION_CHANNEL_CALLBACK = ::core::option::Option<unsafe extern "system" fn(channelinstance: *const ::core::ffi::c_void, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SOAPUDP_URL {
    pub url: WS_URL,
    pub host: WS_STRING,
    pub port: u16,
    pub portAsString: WS_STRING,
    pub path: WS_STRING,
    pub query: WS_STRING,
    pub fragment: WS_STRING,
}
impl ::core::marker::Copy for WS_SOAPUDP_URL {}
impl ::core::clone::Clone for WS_SOAPUDP_URL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SOAPUDP_URL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SOAPUDP_URL").field("url", &self.url).field("host", &self.host).field("port", &self.port).field("portAsString", &self.portAsString).field("path", &self.path).field("query", &self.query).field("fragment", &self.fragment).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_SOAPUDP_URL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_SOAPUDP_URL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_SOAPUDP_URL>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_SOAPUDP_URL {}
impl ::core::default::Default for WS_SOAPUDP_URL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SPN_ENDPOINT_IDENTITY {
    pub identity: WS_ENDPOINT_IDENTITY,
    pub spn: WS_STRING,
}
impl ::core::marker::Copy for WS_SPN_ENDPOINT_IDENTITY {}
impl ::core::clone::Clone for WS_SPN_ENDPOINT_IDENTITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SPN_ENDPOINT_IDENTITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SPN_ENDPOINT_IDENTITY").field("identity", &self.identity).field("spn", &self.spn).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_SPN_ENDPOINT_IDENTITY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_SPN_ENDPOINT_IDENTITY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_SPN_ENDPOINT_IDENTITY>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_SPN_ENDPOINT_IDENTITY {}
impl ::core::default::Default for WS_SPN_ENDPOINT_IDENTITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SSL_TRANSPORT_SECURITY_BINDING {
    pub binding: WS_SECURITY_BINDING,
    pub localCertCredential: *mut WS_CERT_CREDENTIAL,
}
impl ::core::marker::Copy for WS_SSL_TRANSPORT_SECURITY_BINDING {}
impl ::core::clone::Clone for WS_SSL_TRANSPORT_SECURITY_BINDING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SSL_TRANSPORT_SECURITY_BINDING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SSL_TRANSPORT_SECURITY_BINDING").field("binding", &self.binding).field("localCertCredential", &self.localCertCredential).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_SSL_TRANSPORT_SECURITY_BINDING {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_SSL_TRANSPORT_SECURITY_BINDING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_SSL_TRANSPORT_SECURITY_BINDING>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_SSL_TRANSPORT_SECURITY_BINDING {}
impl ::core::default::Default for WS_SSL_TRANSPORT_SECURITY_BINDING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT {
    pub bindingConstraint: WS_SECURITY_BINDING_CONSTRAINT,
    pub out: WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT").field("bindingConstraint", &self.bindingConstraint).field("out", &self.out).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT_0 {
    pub clientCertCredentialRequired: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT_0").field("clientCertCredentialRequired", &self.clientCertCredentialRequired).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SSL_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION {
    pub securityBindingProperties: WS_SECURITY_BINDING_PROPERTIES,
}
impl ::core::marker::Copy for WS_SSL_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_SSL_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SSL_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SSL_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION").field("securityBindingProperties", &self.securityBindingProperties).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_SSL_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_SSL_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_SSL_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_SSL_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION {}
impl ::core::default::Default for WS_SSL_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SSL_TRANSPORT_SECURITY_BINDING_TEMPLATE {
    pub securityBindingProperties: WS_SECURITY_BINDING_PROPERTIES,
    pub localCertCredential: *mut WS_CERT_CREDENTIAL,
}
impl ::core::marker::Copy for WS_SSL_TRANSPORT_SECURITY_BINDING_TEMPLATE {}
impl ::core::clone::Clone for WS_SSL_TRANSPORT_SECURITY_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SSL_TRANSPORT_SECURITY_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SSL_TRANSPORT_SECURITY_BINDING_TEMPLATE").field("securityBindingProperties", &self.securityBindingProperties).field("localCertCredential", &self.localCertCredential).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_SSL_TRANSPORT_SECURITY_BINDING_TEMPLATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_SSL_TRANSPORT_SECURITY_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_SSL_TRANSPORT_SECURITY_BINDING_TEMPLATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_SSL_TRANSPORT_SECURITY_BINDING_TEMPLATE {}
impl ::core::default::Default for WS_SSL_TRANSPORT_SECURITY_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SSPI_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION {
    pub securityBindingProperties: WS_SECURITY_BINDING_PROPERTIES,
}
impl ::core::marker::Copy for WS_SSPI_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_SSPI_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SSPI_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SSPI_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION").field("securityBindingProperties", &self.securityBindingProperties).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_SSPI_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_SSPI_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_SSPI_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_SSPI_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION {}
impl ::core::default::Default for WS_SSPI_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_STRING {
    pub length: u32,
    pub chars: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for WS_STRING {}
impl ::core::clone::Clone for WS_STRING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_STRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_STRING").field("length", &self.length).field("chars", &self.chars).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_STRING {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_STRING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_STRING>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_STRING {}
impl ::core::default::Default for WS_STRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_STRING_DESCRIPTION {
    pub minCharCount: u32,
    pub maxCharCount: u32,
}
impl ::core::marker::Copy for WS_STRING_DESCRIPTION {}
impl ::core::clone::Clone for WS_STRING_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_STRING_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_STRING_DESCRIPTION").field("minCharCount", &self.minCharCount).field("maxCharCount", &self.maxCharCount).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_STRING_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_STRING_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_STRING_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_STRING_DESCRIPTION {}
impl ::core::default::Default for WS_STRING_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_STRING_USERNAME_CREDENTIAL {
    pub credential: WS_USERNAME_CREDENTIAL,
    pub username: WS_STRING,
    pub password: WS_STRING,
}
impl ::core::marker::Copy for WS_STRING_USERNAME_CREDENTIAL {}
impl ::core::clone::Clone for WS_STRING_USERNAME_CREDENTIAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_STRING_USERNAME_CREDENTIAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_STRING_USERNAME_CREDENTIAL").field("credential", &self.credential).field("username", &self.username).field("password", &self.password).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_STRING_USERNAME_CREDENTIAL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_STRING_USERNAME_CREDENTIAL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_STRING_USERNAME_CREDENTIAL>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_STRING_USERNAME_CREDENTIAL {}
impl ::core::default::Default for WS_STRING_USERNAME_CREDENTIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_STRING_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    pub credential: WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL,
    pub username: WS_STRING,
    pub password: WS_STRING,
    pub domain: WS_STRING,
}
impl ::core::marker::Copy for WS_STRING_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {}
impl ::core::clone::Clone for WS_STRING_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_STRING_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_STRING_WINDOWS_INTEGRATED_AUTH_CREDENTIAL").field("credential", &self.credential).field("username", &self.username).field("password", &self.password).field("domain", &self.domain).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_STRING_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_STRING_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_STRING_WINDOWS_INTEGRATED_AUTH_CREDENTIAL>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_STRING_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {}
impl ::core::default::Default for WS_STRING_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_STRUCT_ABSTRACT: i32 = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_STRUCT_DESCRIPTION {
    pub size: u32,
    pub alignment: u32,
    pub fields: *mut *mut WS_FIELD_DESCRIPTION,
    pub fieldCount: u32,
    pub typeLocalName: *mut WS_XML_STRING,
    pub typeNs: *mut WS_XML_STRING,
    pub parentType: *mut WS_STRUCT_DESCRIPTION,
    pub subTypes: *mut *mut WS_STRUCT_DESCRIPTION,
    pub subTypeCount: u32,
    pub structOptions: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_STRUCT_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_STRUCT_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_STRUCT_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_STRUCT_DESCRIPTION").field("size", &self.size).field("alignment", &self.alignment).field("fields", &self.fields).field("fieldCount", &self.fieldCount).field("typeLocalName", &self.typeLocalName).field("typeNs", &self.typeNs).field("parentType", &self.parentType).field("subTypes", &self.subTypes).field("subTypeCount", &self.subTypeCount).field("structOptions", &self.structOptions).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_STRUCT_DESCRIPTION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_STRUCT_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_STRUCT_DESCRIPTION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_STRUCT_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_STRUCT_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_STRUCT_IGNORE_TRAILING_ELEMENT_CONTENT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_STRUCT_IGNORE_UNHANDLED_ATTRIBUTES: i32 = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SUBJECT_NAME_CERT_CREDENTIAL {
    pub credential: WS_CERT_CREDENTIAL,
    pub storeLocation: u32,
    pub storeName: WS_STRING,
    pub subjectName: WS_STRING,
}
impl ::core::marker::Copy for WS_SUBJECT_NAME_CERT_CREDENTIAL {}
impl ::core::clone::Clone for WS_SUBJECT_NAME_CERT_CREDENTIAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SUBJECT_NAME_CERT_CREDENTIAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SUBJECT_NAME_CERT_CREDENTIAL").field("credential", &self.credential).field("storeLocation", &self.storeLocation).field("storeName", &self.storeName).field("subjectName", &self.subjectName).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_SUBJECT_NAME_CERT_CREDENTIAL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_SUBJECT_NAME_CERT_CREDENTIAL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_SUBJECT_NAME_CERT_CREDENTIAL>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_SUBJECT_NAME_CERT_CREDENTIAL {}
impl ::core::default::Default for WS_SUBJECT_NAME_CERT_CREDENTIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_TCP_BINDING_TEMPLATE {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
}
impl ::core::marker::Copy for WS_TCP_BINDING_TEMPLATE {}
impl ::core::clone::Clone for WS_TCP_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_TCP_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TCP_BINDING_TEMPLATE").field("channelProperties", &self.channelProperties).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_TCP_BINDING_TEMPLATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_TCP_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_TCP_BINDING_TEMPLATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_TCP_BINDING_TEMPLATE {}
impl ::core::default::Default for WS_TCP_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_TCP_POLICY_DESCRIPTION {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
}
impl ::core::marker::Copy for WS_TCP_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_TCP_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_TCP_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TCP_POLICY_DESCRIPTION").field("channelProperties", &self.channelProperties).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_TCP_POLICY_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_TCP_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_TCP_POLICY_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_TCP_POLICY_DESCRIPTION {}
impl ::core::default::Default for WS_TCP_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_TCP_SSPI_BINDING_TEMPLATE {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sspiTransportSecurityBinding: WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_TEMPLATE,
}
impl ::core::marker::Copy for WS_TCP_SSPI_BINDING_TEMPLATE {}
impl ::core::clone::Clone for WS_TCP_SSPI_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_TCP_SSPI_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TCP_SSPI_BINDING_TEMPLATE").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sspiTransportSecurityBinding", &self.sspiTransportSecurityBinding).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_TCP_SSPI_BINDING_TEMPLATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_TCP_SSPI_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_TCP_SSPI_BINDING_TEMPLATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_TCP_SSPI_BINDING_TEMPLATE {}
impl ::core::default::Default for WS_TCP_SSPI_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_TCP_SSPI_KERBEROS_APREQ_BINDING_TEMPLATE {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sspiTransportSecurityBinding: WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_TEMPLATE,
    pub kerberosApreqMessageSecurityBinding: WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_TEMPLATE,
}
impl ::core::marker::Copy for WS_TCP_SSPI_KERBEROS_APREQ_BINDING_TEMPLATE {}
impl ::core::clone::Clone for WS_TCP_SSPI_KERBEROS_APREQ_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_TCP_SSPI_KERBEROS_APREQ_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TCP_SSPI_KERBEROS_APREQ_BINDING_TEMPLATE").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sspiTransportSecurityBinding", &self.sspiTransportSecurityBinding).field("kerberosApreqMessageSecurityBinding", &self.kerberosApreqMessageSecurityBinding).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_TCP_SSPI_KERBEROS_APREQ_BINDING_TEMPLATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_TCP_SSPI_KERBEROS_APREQ_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_TCP_SSPI_KERBEROS_APREQ_BINDING_TEMPLATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_TCP_SSPI_KERBEROS_APREQ_BINDING_TEMPLATE {}
impl ::core::default::Default for WS_TCP_SSPI_KERBEROS_APREQ_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_TCP_SSPI_KERBEROS_APREQ_POLICY_DESCRIPTION {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sspiTransportSecurityBinding: WS_SSPI_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION,
    pub kerberosApreqMessageSecurityBinding: WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION,
}
impl ::core::marker::Copy for WS_TCP_SSPI_KERBEROS_APREQ_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_TCP_SSPI_KERBEROS_APREQ_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_TCP_SSPI_KERBEROS_APREQ_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TCP_SSPI_KERBEROS_APREQ_POLICY_DESCRIPTION").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sspiTransportSecurityBinding", &self.sspiTransportSecurityBinding).field("kerberosApreqMessageSecurityBinding", &self.kerberosApreqMessageSecurityBinding).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_TCP_SSPI_KERBEROS_APREQ_POLICY_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_TCP_SSPI_KERBEROS_APREQ_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_TCP_SSPI_KERBEROS_APREQ_POLICY_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_TCP_SSPI_KERBEROS_APREQ_POLICY_DESCRIPTION {}
impl ::core::default::Default for WS_TCP_SSPI_KERBEROS_APREQ_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sspiTransportSecurityBinding: WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_TEMPLATE,
    pub kerberosApreqMessageSecurityBinding: WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_TEMPLATE,
    pub securityContextSecurityBinding: WS_SECURITY_CONTEXT_SECURITY_BINDING_TEMPLATE,
}
impl ::core::marker::Copy for WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE {}
impl ::core::clone::Clone for WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sspiTransportSecurityBinding", &self.sspiTransportSecurityBinding).field("kerberosApreqMessageSecurityBinding", &self.kerberosApreqMessageSecurityBinding).field("securityContextSecurityBinding", &self.securityContextSecurityBinding).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE {}
impl ::core::default::Default for WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sspiTransportSecurityBinding: WS_SSPI_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION,
    pub kerberosApreqMessageSecurityBinding: WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION,
    pub securityContextSecurityBinding: WS_SECURITY_CONTEXT_SECURITY_BINDING_POLICY_DESCRIPTION,
}
impl ::core::marker::Copy for WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sspiTransportSecurityBinding", &self.sspiTransportSecurityBinding).field("kerberosApreqMessageSecurityBinding", &self.kerberosApreqMessageSecurityBinding).field("securityContextSecurityBinding", &self.securityContextSecurityBinding).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION {}
impl ::core::default::Default for WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_TCP_SSPI_POLICY_DESCRIPTION {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sspiTransportSecurityBinding: WS_SSPI_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION,
}
impl ::core::marker::Copy for WS_TCP_SSPI_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_TCP_SSPI_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_TCP_SSPI_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TCP_SSPI_POLICY_DESCRIPTION").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sspiTransportSecurityBinding", &self.sspiTransportSecurityBinding).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_TCP_SSPI_POLICY_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_TCP_SSPI_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_TCP_SSPI_POLICY_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_TCP_SSPI_POLICY_DESCRIPTION {}
impl ::core::default::Default for WS_TCP_SSPI_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING {
    pub binding: WS_SECURITY_BINDING,
    pub clientCredential: *mut WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL,
}
impl ::core::marker::Copy for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING {}
impl ::core::clone::Clone for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING").field("binding", &self.binding).field("clientCredential", &self.clientCredential).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING {}
impl ::core::default::Default for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_CONSTRAINT {
    pub bindingConstraint: WS_SECURITY_BINDING_CONSTRAINT,
}
impl ::core::marker::Copy for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_CONSTRAINT {}
impl ::core::clone::Clone for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_CONSTRAINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_CONSTRAINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_CONSTRAINT").field("bindingConstraint", &self.bindingConstraint).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_CONSTRAINT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_CONSTRAINT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_CONSTRAINT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_CONSTRAINT {}
impl ::core::default::Default for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_CONSTRAINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_TEMPLATE {
    pub securityBindingProperties: WS_SECURITY_BINDING_PROPERTIES,
    pub clientCredential: *mut WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL,
}
impl ::core::marker::Copy for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_TEMPLATE {}
impl ::core::clone::Clone for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_TEMPLATE").field("securityBindingProperties", &self.securityBindingProperties).field("clientCredential", &self.clientCredential).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_TEMPLATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_TEMPLATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_TEMPLATE {}
impl ::core::default::Default for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_TCP_SSPI_USERNAME_BINDING_TEMPLATE {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sspiTransportSecurityBinding: WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_TEMPLATE,
    pub usernameMessageSecurityBinding: WS_USERNAME_MESSAGE_SECURITY_BINDING_TEMPLATE,
}
impl ::core::marker::Copy for WS_TCP_SSPI_USERNAME_BINDING_TEMPLATE {}
impl ::core::clone::Clone for WS_TCP_SSPI_USERNAME_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_TCP_SSPI_USERNAME_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TCP_SSPI_USERNAME_BINDING_TEMPLATE").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sspiTransportSecurityBinding", &self.sspiTransportSecurityBinding).field("usernameMessageSecurityBinding", &self.usernameMessageSecurityBinding).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_TCP_SSPI_USERNAME_BINDING_TEMPLATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_TCP_SSPI_USERNAME_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_TCP_SSPI_USERNAME_BINDING_TEMPLATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_TCP_SSPI_USERNAME_BINDING_TEMPLATE {}
impl ::core::default::Default for WS_TCP_SSPI_USERNAME_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_TCP_SSPI_USERNAME_POLICY_DESCRIPTION {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sspiTransportSecurityBinding: WS_SSPI_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION,
    pub usernameMessageSecurityBinding: WS_USERNAME_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION,
}
impl ::core::marker::Copy for WS_TCP_SSPI_USERNAME_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_TCP_SSPI_USERNAME_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_TCP_SSPI_USERNAME_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TCP_SSPI_USERNAME_POLICY_DESCRIPTION").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sspiTransportSecurityBinding", &self.sspiTransportSecurityBinding).field("usernameMessageSecurityBinding", &self.usernameMessageSecurityBinding).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_TCP_SSPI_USERNAME_POLICY_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_TCP_SSPI_USERNAME_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_TCP_SSPI_USERNAME_POLICY_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_TCP_SSPI_USERNAME_POLICY_DESCRIPTION {}
impl ::core::default::Default for WS_TCP_SSPI_USERNAME_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sspiTransportSecurityBinding: WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_TEMPLATE,
    pub usernameMessageSecurityBinding: WS_USERNAME_MESSAGE_SECURITY_BINDING_TEMPLATE,
    pub securityContextSecurityBinding: WS_SECURITY_CONTEXT_SECURITY_BINDING_TEMPLATE,
}
impl ::core::marker::Copy for WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE {}
impl ::core::clone::Clone for WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sspiTransportSecurityBinding", &self.sspiTransportSecurityBinding).field("usernameMessageSecurityBinding", &self.usernameMessageSecurityBinding).field("securityContextSecurityBinding", &self.securityContextSecurityBinding).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE {}
impl ::core::default::Default for WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sspiTransportSecurityBinding: WS_SSPI_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION,
    pub usernameMessageSecurityBinding: WS_USERNAME_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION,
    pub securityContextSecurityBinding: WS_SECURITY_CONTEXT_SECURITY_BINDING_POLICY_DESCRIPTION,
}
impl ::core::marker::Copy for WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sspiTransportSecurityBinding", &self.sspiTransportSecurityBinding).field("usernameMessageSecurityBinding", &self.usernameMessageSecurityBinding).field("securityContextSecurityBinding", &self.securityContextSecurityBinding).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION {}
impl ::core::default::Default for WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_THUMBPRINT_CERT_CREDENTIAL {
    pub credential: WS_CERT_CREDENTIAL,
    pub storeLocation: u32,
    pub storeName: WS_STRING,
    pub thumbprint: WS_STRING,
}
impl ::core::marker::Copy for WS_THUMBPRINT_CERT_CREDENTIAL {}
impl ::core::clone::Clone for WS_THUMBPRINT_CERT_CREDENTIAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_THUMBPRINT_CERT_CREDENTIAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_THUMBPRINT_CERT_CREDENTIAL").field("credential", &self.credential).field("storeLocation", &self.storeLocation).field("storeName", &self.storeName).field("thumbprint", &self.thumbprint).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_THUMBPRINT_CERT_CREDENTIAL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_THUMBPRINT_CERT_CREDENTIAL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_THUMBPRINT_CERT_CREDENTIAL>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_THUMBPRINT_CERT_CREDENTIAL {}
impl ::core::default::Default for WS_THUMBPRINT_CERT_CREDENTIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_TIMESPAN {
    pub ticks: i64,
}
impl ::core::marker::Copy for WS_TIMESPAN {}
impl ::core::clone::Clone for WS_TIMESPAN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_TIMESPAN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TIMESPAN").field("ticks", &self.ticks).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_TIMESPAN {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_TIMESPAN {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_TIMESPAN>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_TIMESPAN {}
impl ::core::default::Default for WS_TIMESPAN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_TIMESPAN_DESCRIPTION {
    pub minValue: WS_TIMESPAN,
    pub maxValue: WS_TIMESPAN,
}
impl ::core::marker::Copy for WS_TIMESPAN_DESCRIPTION {}
impl ::core::clone::Clone for WS_TIMESPAN_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_TIMESPAN_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TIMESPAN_DESCRIPTION").field("minValue", &self.minValue).field("maxValue", &self.maxValue).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_TIMESPAN_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_TIMESPAN_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_TIMESPAN_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_TIMESPAN_DESCRIPTION {}
impl ::core::default::Default for WS_TIMESPAN_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_TRACE_API(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_NONE: WS_TRACE_API = WS_TRACE_API(-1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_START_READER_CANONICALIZATION: WS_TRACE_API = WS_TRACE_API(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_END_READER_CANONICALIZATION: WS_TRACE_API = WS_TRACE_API(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_START_WRITER_CANONICALIZATION: WS_TRACE_API = WS_TRACE_API(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_END_WRITER_CANONICALIZATION: WS_TRACE_API = WS_TRACE_API(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_CREATE_XML_BUFFER: WS_TRACE_API = WS_TRACE_API(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_REMOVE_NODE: WS_TRACE_API = WS_TRACE_API(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_CREATE_READER: WS_TRACE_API = WS_TRACE_API(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_SET_INPUT: WS_TRACE_API = WS_TRACE_API(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_SET_INPUT_TO_BUFFER: WS_TRACE_API = WS_TRACE_API(8i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_FREE_XML_READER: WS_TRACE_API = WS_TRACE_API(9i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_READER_PROPERTY: WS_TRACE_API = WS_TRACE_API(10i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_READER_NODE: WS_TRACE_API = WS_TRACE_API(11i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_FILL_READER: WS_TRACE_API = WS_TRACE_API(12i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_START_ELEMENT: WS_TRACE_API = WS_TRACE_API(13i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_TO_START_ELEMENT: WS_TRACE_API = WS_TRACE_API(14i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_START_ATTRIBUTE: WS_TRACE_API = WS_TRACE_API(15i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_END_ATTRIBUTE: WS_TRACE_API = WS_TRACE_API(16i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_NODE: WS_TRACE_API = WS_TRACE_API(17i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_SKIP_NODE: WS_TRACE_API = WS_TRACE_API(18i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_END_ELEMENT: WS_TRACE_API = WS_TRACE_API(19i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_FIND_ATTRIBUTE: WS_TRACE_API = WS_TRACE_API(20i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_ELEMENT_VALUE: WS_TRACE_API = WS_TRACE_API(21i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_CHARS: WS_TRACE_API = WS_TRACE_API(22i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_CHARS_UTF8: WS_TRACE_API = WS_TRACE_API(23i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_BYTES: WS_TRACE_API = WS_TRACE_API(24i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_ARRAY: WS_TRACE_API = WS_TRACE_API(25i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_READER_POSITION: WS_TRACE_API = WS_TRACE_API(26i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_SET_READER_POSITION: WS_TRACE_API = WS_TRACE_API(27i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_MOVE_READER: WS_TRACE_API = WS_TRACE_API(28i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_CREATE_WRITER: WS_TRACE_API = WS_TRACE_API(29i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_FREE_XML_WRITER: WS_TRACE_API = WS_TRACE_API(30i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_SET_OUTPUT: WS_TRACE_API = WS_TRACE_API(31i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_SET_OUTPUT_TO_BUFFER: WS_TRACE_API = WS_TRACE_API(32i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_WRITER_PROPERTY: WS_TRACE_API = WS_TRACE_API(33i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_FLUSH_WRITER: WS_TRACE_API = WS_TRACE_API(34i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_START_ELEMENT: WS_TRACE_API = WS_TRACE_API(35i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_END_START_ELEMENT: WS_TRACE_API = WS_TRACE_API(36i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_XMLNS_ATTRIBUTE: WS_TRACE_API = WS_TRACE_API(37i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_START_ATTRIBUTE: WS_TRACE_API = WS_TRACE_API(38i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_END_ATTRIBUTE: WS_TRACE_API = WS_TRACE_API(39i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_VALUE: WS_TRACE_API = WS_TRACE_API(40i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_XML_BUFFER: WS_TRACE_API = WS_TRACE_API(41i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_XML_BUFFER: WS_TRACE_API = WS_TRACE_API(42i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_XML_BUFFER_TO_BYTES: WS_TRACE_API = WS_TRACE_API(43i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_XML_BUFFER_FROM_BYTES: WS_TRACE_API = WS_TRACE_API(44i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_ARRAY: WS_TRACE_API = WS_TRACE_API(45i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_QUALIFIED_NAME: WS_TRACE_API = WS_TRACE_API(46i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_CHARS: WS_TRACE_API = WS_TRACE_API(47i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_CHARS_UTF8: WS_TRACE_API = WS_TRACE_API(48i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_BYTES: WS_TRACE_API = WS_TRACE_API(49i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_PUSH_BYTES: WS_TRACE_API = WS_TRACE_API(50i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_PULL_BYTES: WS_TRACE_API = WS_TRACE_API(51i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_END_ELEMENT: WS_TRACE_API = WS_TRACE_API(52i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_TEXT: WS_TRACE_API = WS_TRACE_API(53i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_START_CDATA: WS_TRACE_API = WS_TRACE_API(54i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_END_CDATA: WS_TRACE_API = WS_TRACE_API(55i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_NODE: WS_TRACE_API = WS_TRACE_API(56i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_PREFIX_FROM_NAMESPACE: WS_TRACE_API = WS_TRACE_API(57i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_WRITER_POSITION: WS_TRACE_API = WS_TRACE_API(58i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_SET_WRITER_POSITION: WS_TRACE_API = WS_TRACE_API(59i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_MOVE_WRITER: WS_TRACE_API = WS_TRACE_API(60i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_TRIM_XML_WHITESPACE: WS_TRACE_API = WS_TRACE_API(61i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_VERIFY_XML_NCNAME: WS_TRACE_API = WS_TRACE_API(62i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_XML_STRING_EQUALS: WS_TRACE_API = WS_TRACE_API(63i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_NAMESPACE_FROM_PREFIX: WS_TRACE_API = WS_TRACE_API(64i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_QUALIFIED_NAME: WS_TRACE_API = WS_TRACE_API(65i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_XML_ATTRIBUTE: WS_TRACE_API = WS_TRACE_API(66i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_COPY_NODE: WS_TRACE_API = WS_TRACE_API(67i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_ASYNC_EXECUTE: WS_TRACE_API = WS_TRACE_API(68i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_CREATE_CHANNEL: WS_TRACE_API = WS_TRACE_API(69i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_OPEN_CHANNEL: WS_TRACE_API = WS_TRACE_API(70i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_SEND_MESSAGE: WS_TRACE_API = WS_TRACE_API(71i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_RECEIVE_MESSAGE: WS_TRACE_API = WS_TRACE_API(72i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_REQUEST_REPLY: WS_TRACE_API = WS_TRACE_API(73i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_SEND_REPLY_MESSAGE: WS_TRACE_API = WS_TRACE_API(74i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_SEND_FAULT_MESSAGE_FOR_ERROR: WS_TRACE_API = WS_TRACE_API(75i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_CHANNEL_PROPERTY: WS_TRACE_API = WS_TRACE_API(76i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_SET_CHANNEL_PROPERTY: WS_TRACE_API = WS_TRACE_API(77i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_MESSAGE_START: WS_TRACE_API = WS_TRACE_API(78i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_MESSAGE_END: WS_TRACE_API = WS_TRACE_API(79i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_MESSAGE_START: WS_TRACE_API = WS_TRACE_API(80i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_MESSAGE_END: WS_TRACE_API = WS_TRACE_API(81i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_CLOSE_CHANNEL: WS_TRACE_API = WS_TRACE_API(82i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_ABORT_CHANNEL: WS_TRACE_API = WS_TRACE_API(83i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_FREE_CHANNEL: WS_TRACE_API = WS_TRACE_API(84i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_RESET_CHANNEL: WS_TRACE_API = WS_TRACE_API(85i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_ABANDON_MESSAGE: WS_TRACE_API = WS_TRACE_API(86i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_SHUTDOWN_SESSION_CHANNEL: WS_TRACE_API = WS_TRACE_API(87i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_CONTEXT_PROPERTY: WS_TRACE_API = WS_TRACE_API(88i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_DICTIONARY: WS_TRACE_API = WS_TRACE_API(89i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_ENDPOINT_ADDRESS_EXTENSION: WS_TRACE_API = WS_TRACE_API(90i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_CREATE_ERROR: WS_TRACE_API = WS_TRACE_API(91i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_ADD_ERROR_STRING: WS_TRACE_API = WS_TRACE_API(92i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_ERROR_STRING: WS_TRACE_API = WS_TRACE_API(93i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_COPY_ERROR: WS_TRACE_API = WS_TRACE_API(94i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_ERROR_PROPERTY: WS_TRACE_API = WS_TRACE_API(95i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_SET_ERROR_PROPERTY: WS_TRACE_API = WS_TRACE_API(96i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_RESET_ERROR: WS_TRACE_API = WS_TRACE_API(97i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_FREE_ERROR: WS_TRACE_API = WS_TRACE_API(98i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_FAULT_ERROR_PROPERTY: WS_TRACE_API = WS_TRACE_API(99i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_SET_FAULT_ERROR_PROPERTY: WS_TRACE_API = WS_TRACE_API(100i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_CREATE_FAULT_FROM_ERROR: WS_TRACE_API = WS_TRACE_API(101i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_SET_FAULT_ERROR_DETAIL: WS_TRACE_API = WS_TRACE_API(102i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_FAULT_ERROR_DETAIL: WS_TRACE_API = WS_TRACE_API(103i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_CREATE_HEAP: WS_TRACE_API = WS_TRACE_API(104i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_ALLOC: WS_TRACE_API = WS_TRACE_API(105i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_HEAP_PROPERTY: WS_TRACE_API = WS_TRACE_API(106i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_RESET_HEAP: WS_TRACE_API = WS_TRACE_API(107i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_FREE_HEAP: WS_TRACE_API = WS_TRACE_API(108i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_CREATE_LISTENER: WS_TRACE_API = WS_TRACE_API(109i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_OPEN_LISTENER: WS_TRACE_API = WS_TRACE_API(110i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_ACCEPT_CHANNEL: WS_TRACE_API = WS_TRACE_API(111i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_CLOSE_LISTENER: WS_TRACE_API = WS_TRACE_API(112i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_ABORT_LISTENER: WS_TRACE_API = WS_TRACE_API(113i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_RESET_LISTENER: WS_TRACE_API = WS_TRACE_API(114i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_FREE_LISTENER: WS_TRACE_API = WS_TRACE_API(115i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_LISTENER_PROPERTY: WS_TRACE_API = WS_TRACE_API(116i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_SET_LISTENER_PROPERTY: WS_TRACE_API = WS_TRACE_API(117i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_CREATE_CHANNEL_FOR_LISTENER: WS_TRACE_API = WS_TRACE_API(118i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_CREATE_MESSAGE: WS_TRACE_API = WS_TRACE_API(119i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_CREATE_MESSAGE_FOR_CHANNEL: WS_TRACE_API = WS_TRACE_API(120i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_INITIALIZE_MESSAGE: WS_TRACE_API = WS_TRACE_API(121i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_RESET_MESSAGE: WS_TRACE_API = WS_TRACE_API(122i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_FREE_MESSAGE: WS_TRACE_API = WS_TRACE_API(123i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_HEADER_ATTRIBUTES: WS_TRACE_API = WS_TRACE_API(124i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_HEADER: WS_TRACE_API = WS_TRACE_API(125i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_CUSTOM_HEADER: WS_TRACE_API = WS_TRACE_API(126i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_REMOVE_HEADER: WS_TRACE_API = WS_TRACE_API(127i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_SET_HEADER: WS_TRACE_API = WS_TRACE_API(128i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_REMOVE_CUSTOM_HEADER: WS_TRACE_API = WS_TRACE_API(129i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_ADD_CUSTOM_HEADER: WS_TRACE_API = WS_TRACE_API(130i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_ADD_MAPPED_HEADER: WS_TRACE_API = WS_TRACE_API(131i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_REMOVE_MAPPED_HEADER: WS_TRACE_API = WS_TRACE_API(132i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_MAPPED_HEADER: WS_TRACE_API = WS_TRACE_API(133i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_BODY: WS_TRACE_API = WS_TRACE_API(134i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_BODY: WS_TRACE_API = WS_TRACE_API(135i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_ENVELOPE_START: WS_TRACE_API = WS_TRACE_API(136i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_ENVELOPE_END: WS_TRACE_API = WS_TRACE_API(137i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_ENVELOPE_START: WS_TRACE_API = WS_TRACE_API(138i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_ENVELOPE_END: WS_TRACE_API = WS_TRACE_API(139i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_MESSAGE_PROPERTY: WS_TRACE_API = WS_TRACE_API(140i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_SET_MESSAGE_PROPERTY: WS_TRACE_API = WS_TRACE_API(141i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_ADDRESS_MESSAGE: WS_TRACE_API = WS_TRACE_API(142i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_CHECK_MUST_UNDERSTAND_HEADERS: WS_TRACE_API = WS_TRACE_API(143i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_MARK_HEADER_AS_UNDERSTOOD: WS_TRACE_API = WS_TRACE_API(144i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_FILL_BODY: WS_TRACE_API = WS_TRACE_API(145i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_FLUSH_BODY: WS_TRACE_API = WS_TRACE_API(146i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_REQUEST_SECURITY_TOKEN: WS_TRACE_API = WS_TRACE_API(147i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_SECURITY_TOKEN_PROPERTY: WS_TRACE_API = WS_TRACE_API(148i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_CREATE_XML_SECURITY_TOKEN: WS_TRACE_API = WS_TRACE_API(149i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_FREE_SECURITY_TOKEN: WS_TRACE_API = WS_TRACE_API(150i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_REVOKE_SECURITY_CONTEXT: WS_TRACE_API = WS_TRACE_API(151i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_SECURITY_CONTEXT_PROPERTY: WS_TRACE_API = WS_TRACE_API(152i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_ELEMENT_TYPE: WS_TRACE_API = WS_TRACE_API(153i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_ATTRIBUTE_TYPE: WS_TRACE_API = WS_TRACE_API(154i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_TYPE: WS_TRACE_API = WS_TRACE_API(155i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_ELEMENT_TYPE: WS_TRACE_API = WS_TRACE_API(156i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_ATTRIBUTE_TYPE: WS_TRACE_API = WS_TRACE_API(157i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_TYPE: WS_TRACE_API = WS_TRACE_API(158i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_SERVICE_REGISTER_FOR_CANCEL: WS_TRACE_API = WS_TRACE_API(159i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_SERVICE_HOST_PROPERTY: WS_TRACE_API = WS_TRACE_API(160i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_CREATE_SERVICE_HOST: WS_TRACE_API = WS_TRACE_API(161i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_OPEN_SERVICE_HOST: WS_TRACE_API = WS_TRACE_API(162i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_CLOSE_SERVICE_HOST: WS_TRACE_API = WS_TRACE_API(163i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_ABORT_SERVICE_HOST: WS_TRACE_API = WS_TRACE_API(164i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_FREE_SERVICE_HOST: WS_TRACE_API = WS_TRACE_API(165i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_RESET_SERVICE_HOST: WS_TRACE_API = WS_TRACE_API(166i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_SERVICE_PROXY_PROPERTY: WS_TRACE_API = WS_TRACE_API(167i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_CREATE_SERVICE_PROXY: WS_TRACE_API = WS_TRACE_API(168i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_OPEN_SERVICE_PROXY: WS_TRACE_API = WS_TRACE_API(169i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_CLOSE_SERVICE_PROXY: WS_TRACE_API = WS_TRACE_API(170i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_ABORT_SERVICE_PROXY: WS_TRACE_API = WS_TRACE_API(171i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_FREE_SERVICE_PROXY: WS_TRACE_API = WS_TRACE_API(172i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_RESET_SERVICE_PROXY: WS_TRACE_API = WS_TRACE_API(173i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_ABORT_CALL: WS_TRACE_API = WS_TRACE_API(174i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_CALL: WS_TRACE_API = WS_TRACE_API(175i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_DECODE_URL: WS_TRACE_API = WS_TRACE_API(176i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_ENCODE_URL: WS_TRACE_API = WS_TRACE_API(177i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_COMBINE_URL: WS_TRACE_API = WS_TRACE_API(178i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_DATETIME_TO_FILETIME: WS_TRACE_API = WS_TRACE_API(179i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_FILETIME_TO_DATETIME: WS_TRACE_API = WS_TRACE_API(180i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_DUMP_MEMORY: WS_TRACE_API = WS_TRACE_API(181i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_SET_AUTOFAIL: WS_TRACE_API = WS_TRACE_API(182i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_CREATE_METADATA: WS_TRACE_API = WS_TRACE_API(183i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_METADATA: WS_TRACE_API = WS_TRACE_API(184i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_FREE_METADATA: WS_TRACE_API = WS_TRACE_API(185i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_RESET_METADATA: WS_TRACE_API = WS_TRACE_API(186i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_METADATA_PROPERTY: WS_TRACE_API = WS_TRACE_API(187i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_MISSING_METADATA_DOCUMENT_ADDRESS: WS_TRACE_API = WS_TRACE_API(188i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_METADATA_ENDPOINTS: WS_TRACE_API = WS_TRACE_API(189i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_MATCH_POLICY_ALTERNATIVE: WS_TRACE_API = WS_TRACE_API(190i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_POLICY_PROPERTY: WS_TRACE_API = WS_TRACE_API(191i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_POLICY_ALTERNATIVE_COUNT: WS_TRACE_API = WS_TRACE_API(192i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WS_CREATE_SERVICE_PROXY_FROM_TEMPLATE: WS_TRACE_API = WS_TRACE_API(193i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WS_CREATE_SERVICE_HOST_FROM_TEMPLATE: WS_TRACE_API = WS_TRACE_API(194i32);
impl ::core::marker::Copy for WS_TRACE_API {}
impl ::core::clone::Clone for WS_TRACE_API {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_TRACE_API {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_TRACE_API {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_TRACE_API {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_TRACE_API").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_TRANSFER_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_STREAMED_INPUT_TRANSFER_MODE: WS_TRANSFER_MODE = WS_TRANSFER_MODE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_STREAMED_OUTPUT_TRANSFER_MODE: WS_TRANSFER_MODE = WS_TRANSFER_MODE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_BUFFERED_TRANSFER_MODE: WS_TRANSFER_MODE = WS_TRANSFER_MODE(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_STREAMED_TRANSFER_MODE: WS_TRANSFER_MODE = WS_TRANSFER_MODE(3i32);
impl ::core::marker::Copy for WS_TRANSFER_MODE {}
impl ::core::clone::Clone for WS_TRANSFER_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_TRANSFER_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_TRANSFER_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_TRANSFER_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_TRANSFER_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_TRUST_VERSION(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRUST_VERSION_FEBRUARY_2005: WS_TRUST_VERSION = WS_TRUST_VERSION(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRUST_VERSION_1_3: WS_TRUST_VERSION = WS_TRUST_VERSION(2i32);
impl ::core::marker::Copy for WS_TRUST_VERSION {}
impl ::core::clone::Clone for WS_TRUST_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_TRUST_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_TRUST_VERSION {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_TRUST_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_TRUST_VERSION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_BOOL_TYPE: WS_TYPE = WS_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_INT8_TYPE: WS_TYPE = WS_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_INT16_TYPE: WS_TYPE = WS_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_INT32_TYPE: WS_TYPE = WS_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_INT64_TYPE: WS_TYPE = WS_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_UINT8_TYPE: WS_TYPE = WS_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_UINT16_TYPE: WS_TYPE = WS_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_UINT32_TYPE: WS_TYPE = WS_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_UINT64_TYPE: WS_TYPE = WS_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_FLOAT_TYPE: WS_TYPE = WS_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_DOUBLE_TYPE: WS_TYPE = WS_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_DECIMAL_TYPE: WS_TYPE = WS_TYPE(11i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_DATETIME_TYPE: WS_TYPE = WS_TYPE(12i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TIMESPAN_TYPE: WS_TYPE = WS_TYPE(13i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_GUID_TYPE: WS_TYPE = WS_TYPE(14i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_UNIQUE_ID_TYPE: WS_TYPE = WS_TYPE(15i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_STRING_TYPE: WS_TYPE = WS_TYPE(16i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_WSZ_TYPE: WS_TYPE = WS_TYPE(17i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_BYTES_TYPE: WS_TYPE = WS_TYPE(18i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_STRING_TYPE: WS_TYPE = WS_TYPE(19i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_QNAME_TYPE: WS_TYPE = WS_TYPE(20i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_BUFFER_TYPE: WS_TYPE = WS_TYPE(21i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHAR_ARRAY_TYPE: WS_TYPE = WS_TYPE(22i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_UTF8_ARRAY_TYPE: WS_TYPE = WS_TYPE(23i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_BYTE_ARRAY_TYPE: WS_TYPE = WS_TYPE(24i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_DESCRIPTION_TYPE: WS_TYPE = WS_TYPE(25i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_STRUCT_TYPE: WS_TYPE = WS_TYPE(26i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CUSTOM_TYPE: WS_TYPE = WS_TYPE(27i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ENDPOINT_ADDRESS_TYPE: WS_TYPE = WS_TYPE(28i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_FAULT_TYPE: WS_TYPE = WS_TYPE(29i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_VOID_TYPE: WS_TYPE = WS_TYPE(30i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ENUM_TYPE: WS_TYPE = WS_TYPE(31i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_DURATION_TYPE: WS_TYPE = WS_TYPE(32i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_UNION_TYPE: WS_TYPE = WS_TYPE(33i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ANY_ATTRIBUTES_TYPE: WS_TYPE = WS_TYPE(34i32);
impl ::core::marker::Copy for WS_TYPE {}
impl ::core::clone::Clone for WS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_TYPE_MAPPING(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ELEMENT_TYPE_MAPPING: WS_TYPE_MAPPING = WS_TYPE_MAPPING(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ATTRIBUTE_TYPE_MAPPING: WS_TYPE_MAPPING = WS_TYPE_MAPPING(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ELEMENT_CONTENT_TYPE_MAPPING: WS_TYPE_MAPPING = WS_TYPE_MAPPING(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ANY_ELEMENT_TYPE_MAPPING: WS_TYPE_MAPPING = WS_TYPE_MAPPING(4i32);
impl ::core::marker::Copy for WS_TYPE_MAPPING {}
impl ::core::clone::Clone for WS_TYPE_MAPPING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_TYPE_MAPPING {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_TYPE_MAPPING {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_TYPE_MAPPING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_TYPE_MAPPING").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_UINT16_DESCRIPTION {
    pub minValue: u16,
    pub maxValue: u16,
}
impl ::core::marker::Copy for WS_UINT16_DESCRIPTION {}
impl ::core::clone::Clone for WS_UINT16_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_UINT16_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_UINT16_DESCRIPTION").field("minValue", &self.minValue).field("maxValue", &self.maxValue).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_UINT16_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_UINT16_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_UINT16_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_UINT16_DESCRIPTION {}
impl ::core::default::Default for WS_UINT16_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_UINT32_DESCRIPTION {
    pub minValue: u32,
    pub maxValue: u32,
}
impl ::core::marker::Copy for WS_UINT32_DESCRIPTION {}
impl ::core::clone::Clone for WS_UINT32_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_UINT32_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_UINT32_DESCRIPTION").field("minValue", &self.minValue).field("maxValue", &self.maxValue).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_UINT32_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_UINT32_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_UINT32_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_UINT32_DESCRIPTION {}
impl ::core::default::Default for WS_UINT32_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_UINT64_DESCRIPTION {
    pub minValue: u64,
    pub maxValue: u64,
}
impl ::core::marker::Copy for WS_UINT64_DESCRIPTION {}
impl ::core::clone::Clone for WS_UINT64_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_UINT64_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_UINT64_DESCRIPTION").field("minValue", &self.minValue).field("maxValue", &self.maxValue).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_UINT64_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_UINT64_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_UINT64_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_UINT64_DESCRIPTION {}
impl ::core::default::Default for WS_UINT64_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_UINT8_DESCRIPTION {
    pub minValue: u8,
    pub maxValue: u8,
}
impl ::core::marker::Copy for WS_UINT8_DESCRIPTION {}
impl ::core::clone::Clone for WS_UINT8_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_UINT8_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_UINT8_DESCRIPTION").field("minValue", &self.minValue).field("maxValue", &self.maxValue).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_UINT8_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_UINT8_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_UINT8_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_UINT8_DESCRIPTION {}
impl ::core::default::Default for WS_UINT8_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_UNION_DESCRIPTION {
    pub size: u32,
    pub alignment: u32,
    pub fields: *mut *mut WS_UNION_FIELD_DESCRIPTION,
    pub fieldCount: u32,
    pub enumOffset: u32,
    pub noneEnumValue: i32,
    pub valueIndices: *mut u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_UNION_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_UNION_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_UNION_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_UNION_DESCRIPTION").field("size", &self.size).field("alignment", &self.alignment).field("fields", &self.fields).field("fieldCount", &self.fieldCount).field("enumOffset", &self.enumOffset).field("noneEnumValue", &self.noneEnumValue).field("valueIndices", &self.valueIndices).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_UNION_DESCRIPTION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_UNION_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_UNION_DESCRIPTION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_UNION_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_UNION_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_UNION_FIELD_DESCRIPTION {
    pub value: i32,
    pub field: WS_FIELD_DESCRIPTION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_UNION_FIELD_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_UNION_FIELD_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_UNION_FIELD_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_UNION_FIELD_DESCRIPTION").field("value", &self.value).field("field", &self.field).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_UNION_FIELD_DESCRIPTION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_UNION_FIELD_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_UNION_FIELD_DESCRIPTION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_UNION_FIELD_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_UNION_FIELD_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_UNIQUE_ID {
    pub uri: WS_STRING,
    pub guid: ::windows::core::GUID,
}
impl ::core::marker::Copy for WS_UNIQUE_ID {}
impl ::core::clone::Clone for WS_UNIQUE_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_UNIQUE_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_UNIQUE_ID").field("uri", &self.uri).field("guid", &self.guid).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_UNIQUE_ID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_UNIQUE_ID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_UNIQUE_ID>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_UNIQUE_ID {}
impl ::core::default::Default for WS_UNIQUE_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_UNIQUE_ID_DESCRIPTION {
    pub minCharCount: u32,
    pub maxCharCount: u32,
}
impl ::core::marker::Copy for WS_UNIQUE_ID_DESCRIPTION {}
impl ::core::clone::Clone for WS_UNIQUE_ID_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_UNIQUE_ID_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_UNIQUE_ID_DESCRIPTION").field("minCharCount", &self.minCharCount).field("maxCharCount", &self.maxCharCount).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_UNIQUE_ID_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_UNIQUE_ID_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_UNIQUE_ID_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_UNIQUE_ID_DESCRIPTION {}
impl ::core::default::Default for WS_UNIQUE_ID_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_UNKNOWN_ENDPOINT_IDENTITY {
    pub identity: WS_ENDPOINT_IDENTITY,
    pub element: *mut WS_XML_BUFFER,
}
impl ::core::marker::Copy for WS_UNKNOWN_ENDPOINT_IDENTITY {}
impl ::core::clone::Clone for WS_UNKNOWN_ENDPOINT_IDENTITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_UNKNOWN_ENDPOINT_IDENTITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_UNKNOWN_ENDPOINT_IDENTITY").field("identity", &self.identity).field("element", &self.element).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_UNKNOWN_ENDPOINT_IDENTITY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_UNKNOWN_ENDPOINT_IDENTITY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_UNKNOWN_ENDPOINT_IDENTITY>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_UNKNOWN_ENDPOINT_IDENTITY {}
impl ::core::default::Default for WS_UNKNOWN_ENDPOINT_IDENTITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_UPN_ENDPOINT_IDENTITY {
    pub identity: WS_ENDPOINT_IDENTITY,
    pub upn: WS_STRING,
}
impl ::core::marker::Copy for WS_UPN_ENDPOINT_IDENTITY {}
impl ::core::clone::Clone for WS_UPN_ENDPOINT_IDENTITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_UPN_ENDPOINT_IDENTITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_UPN_ENDPOINT_IDENTITY").field("identity", &self.identity).field("upn", &self.upn).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_UPN_ENDPOINT_IDENTITY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_UPN_ENDPOINT_IDENTITY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_UPN_ENDPOINT_IDENTITY>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_UPN_ENDPOINT_IDENTITY {}
impl ::core::default::Default for WS_UPN_ENDPOINT_IDENTITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_URL {
    pub scheme: WS_URL_SCHEME_TYPE,
}
impl ::core::marker::Copy for WS_URL {}
impl ::core::clone::Clone for WS_URL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_URL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_URL").field("scheme", &self.scheme).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_URL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_URL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_URL>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_URL {}
impl ::core::default::Default for WS_URL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_URL_FLAGS_ALLOW_HOST_WILDCARDS: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_URL_FLAGS_NO_PATH_COLLAPSE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_URL_FLAGS_ZERO_TERMINATE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_URL_SCHEME_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_URL_HTTP_SCHEME_TYPE: WS_URL_SCHEME_TYPE = WS_URL_SCHEME_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_URL_HTTPS_SCHEME_TYPE: WS_URL_SCHEME_TYPE = WS_URL_SCHEME_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_URL_NETTCP_SCHEME_TYPE: WS_URL_SCHEME_TYPE = WS_URL_SCHEME_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_URL_SOAPUDP_SCHEME_TYPE: WS_URL_SCHEME_TYPE = WS_URL_SCHEME_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_URL_NETPIPE_SCHEME_TYPE: WS_URL_SCHEME_TYPE = WS_URL_SCHEME_TYPE(4i32);
impl ::core::marker::Copy for WS_URL_SCHEME_TYPE {}
impl ::core::clone::Clone for WS_URL_SCHEME_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_URL_SCHEME_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_URL_SCHEME_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_URL_SCHEME_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_URL_SCHEME_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_USERNAME_CREDENTIAL {
    pub credentialType: WS_USERNAME_CREDENTIAL_TYPE,
}
impl ::core::marker::Copy for WS_USERNAME_CREDENTIAL {}
impl ::core::clone::Clone for WS_USERNAME_CREDENTIAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_USERNAME_CREDENTIAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_USERNAME_CREDENTIAL").field("credentialType", &self.credentialType).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_USERNAME_CREDENTIAL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_USERNAME_CREDENTIAL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_USERNAME_CREDENTIAL>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_USERNAME_CREDENTIAL {}
impl ::core::default::Default for WS_USERNAME_CREDENTIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_USERNAME_CREDENTIAL_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_STRING_USERNAME_CREDENTIAL_TYPE: WS_USERNAME_CREDENTIAL_TYPE = WS_USERNAME_CREDENTIAL_TYPE(1i32);
impl ::core::marker::Copy for WS_USERNAME_CREDENTIAL_TYPE {}
impl ::core::clone::Clone for WS_USERNAME_CREDENTIAL_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_USERNAME_CREDENTIAL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_USERNAME_CREDENTIAL_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_USERNAME_CREDENTIAL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_USERNAME_CREDENTIAL_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_USERNAME_MESSAGE_SECURITY_BINDING {
    pub binding: WS_SECURITY_BINDING,
    pub bindingUsage: WS_MESSAGE_SECURITY_USAGE,
    pub clientCredential: *mut WS_USERNAME_CREDENTIAL,
    pub passwordValidator: WS_VALIDATE_PASSWORD_CALLBACK,
    pub passwordValidatorCallbackState: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WS_USERNAME_MESSAGE_SECURITY_BINDING {}
impl ::core::clone::Clone for WS_USERNAME_MESSAGE_SECURITY_BINDING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_USERNAME_MESSAGE_SECURITY_BINDING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_USERNAME_MESSAGE_SECURITY_BINDING").field("binding", &self.binding).field("bindingUsage", &self.bindingUsage).field("clientCredential", &self.clientCredential).field("passwordValidator", &self.passwordValidator.map(|f| f as usize)).field("passwordValidatorCallbackState", &self.passwordValidatorCallbackState).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_USERNAME_MESSAGE_SECURITY_BINDING {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_USERNAME_MESSAGE_SECURITY_BINDING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_USERNAME_MESSAGE_SECURITY_BINDING>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_USERNAME_MESSAGE_SECURITY_BINDING {}
impl ::core::default::Default for WS_USERNAME_MESSAGE_SECURITY_BINDING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_USERNAME_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    pub bindingConstraint: WS_SECURITY_BINDING_CONSTRAINT,
    pub bindingUsage: WS_MESSAGE_SECURITY_USAGE,
}
impl ::core::marker::Copy for WS_USERNAME_MESSAGE_SECURITY_BINDING_CONSTRAINT {}
impl ::core::clone::Clone for WS_USERNAME_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_USERNAME_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_USERNAME_MESSAGE_SECURITY_BINDING_CONSTRAINT").field("bindingConstraint", &self.bindingConstraint).field("bindingUsage", &self.bindingUsage).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_USERNAME_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_USERNAME_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_USERNAME_MESSAGE_SECURITY_BINDING_CONSTRAINT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_USERNAME_MESSAGE_SECURITY_BINDING_CONSTRAINT {}
impl ::core::default::Default for WS_USERNAME_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_USERNAME_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {
    pub securityBindingProperties: WS_SECURITY_BINDING_PROPERTIES,
    pub bindingUsage: WS_MESSAGE_SECURITY_USAGE,
}
impl ::core::marker::Copy for WS_USERNAME_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_USERNAME_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_USERNAME_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_USERNAME_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION").field("securityBindingProperties", &self.securityBindingProperties).field("bindingUsage", &self.bindingUsage).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_USERNAME_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_USERNAME_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_USERNAME_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_USERNAME_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {}
impl ::core::default::Default for WS_USERNAME_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_USERNAME_MESSAGE_SECURITY_BINDING_TEMPLATE {
    pub securityBindingProperties: WS_SECURITY_BINDING_PROPERTIES,
    pub clientCredential: *mut WS_USERNAME_CREDENTIAL,
    pub passwordValidator: WS_VALIDATE_PASSWORD_CALLBACK,
    pub passwordValidatorCallbackState: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WS_USERNAME_MESSAGE_SECURITY_BINDING_TEMPLATE {}
impl ::core::clone::Clone for WS_USERNAME_MESSAGE_SECURITY_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_USERNAME_MESSAGE_SECURITY_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_USERNAME_MESSAGE_SECURITY_BINDING_TEMPLATE").field("securityBindingProperties", &self.securityBindingProperties).field("clientCredential", &self.clientCredential).field("passwordValidator", &self.passwordValidator.map(|f| f as usize)).field("passwordValidatorCallbackState", &self.passwordValidatorCallbackState).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_USERNAME_MESSAGE_SECURITY_BINDING_TEMPLATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_USERNAME_MESSAGE_SECURITY_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_USERNAME_MESSAGE_SECURITY_BINDING_TEMPLATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_USERNAME_MESSAGE_SECURITY_BINDING_TEMPLATE {}
impl ::core::default::Default for WS_USERNAME_MESSAGE_SECURITY_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_UTF8_ARRAY_DESCRIPTION {
    pub minByteCount: u32,
    pub maxByteCount: u32,
}
impl ::core::marker::Copy for WS_UTF8_ARRAY_DESCRIPTION {}
impl ::core::clone::Clone for WS_UTF8_ARRAY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_UTF8_ARRAY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_UTF8_ARRAY_DESCRIPTION").field("minByteCount", &self.minByteCount).field("maxByteCount", &self.maxByteCount).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_UTF8_ARRAY_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_UTF8_ARRAY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_UTF8_ARRAY_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_UTF8_ARRAY_DESCRIPTION {}
impl ::core::default::Default for WS_UTF8_ARRAY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_VALIDATE_PASSWORD_CALLBACK = ::core::option::Option<unsafe extern "system" fn(passwordvalidatorcallbackstate: *const ::core::ffi::c_void, username: *const WS_STRING, password: *const WS_STRING, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_VALIDATE_SAML_CALLBACK = ::core::option::Option<unsafe extern "system" fn(samlvalidatorcallbackstate: *const ::core::ffi::c_void, samlassertion: *const WS_XML_BUFFER, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_VALUE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_BOOL_VALUE_TYPE: WS_VALUE_TYPE = WS_VALUE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_INT8_VALUE_TYPE: WS_VALUE_TYPE = WS_VALUE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_INT16_VALUE_TYPE: WS_VALUE_TYPE = WS_VALUE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_INT32_VALUE_TYPE: WS_VALUE_TYPE = WS_VALUE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_INT64_VALUE_TYPE: WS_VALUE_TYPE = WS_VALUE_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_UINT8_VALUE_TYPE: WS_VALUE_TYPE = WS_VALUE_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_UINT16_VALUE_TYPE: WS_VALUE_TYPE = WS_VALUE_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_UINT32_VALUE_TYPE: WS_VALUE_TYPE = WS_VALUE_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_UINT64_VALUE_TYPE: WS_VALUE_TYPE = WS_VALUE_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_FLOAT_VALUE_TYPE: WS_VALUE_TYPE = WS_VALUE_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_DOUBLE_VALUE_TYPE: WS_VALUE_TYPE = WS_VALUE_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_DECIMAL_VALUE_TYPE: WS_VALUE_TYPE = WS_VALUE_TYPE(11i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_DATETIME_VALUE_TYPE: WS_VALUE_TYPE = WS_VALUE_TYPE(12i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TIMESPAN_VALUE_TYPE: WS_VALUE_TYPE = WS_VALUE_TYPE(13i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_GUID_VALUE_TYPE: WS_VALUE_TYPE = WS_VALUE_TYPE(14i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_DURATION_VALUE_TYPE: WS_VALUE_TYPE = WS_VALUE_TYPE(15i32);
impl ::core::marker::Copy for WS_VALUE_TYPE {}
impl ::core::clone::Clone for WS_VALUE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_VALUE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_VALUE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_VALUE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_VALUE_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_VOID_DESCRIPTION {
    pub size: u32,
}
impl ::core::marker::Copy for WS_VOID_DESCRIPTION {}
impl ::core::clone::Clone for WS_VOID_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_VOID_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_VOID_DESCRIPTION").field("size", &self.size).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_VOID_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_VOID_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_VOID_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_VOID_DESCRIPTION {}
impl ::core::default::Default for WS_VOID_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    pub credentialType: WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL_TYPE,
}
impl ::core::marker::Copy for WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {}
impl ::core::clone::Clone for WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL").field("credentialType", &self.credentialType).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {}
impl ::core::default::Default for WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_STRING_WINDOWS_INTEGRATED_AUTH_CREDENTIAL_TYPE: WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL_TYPE = WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_DEFAULT_WINDOWS_INTEGRATED_AUTH_CREDENTIAL_TYPE: WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL_TYPE = WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_OPAQUE_WINDOWS_INTEGRATED_AUTH_CREDENTIAL_TYPE: WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL_TYPE = WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL_TYPE(3i32);
impl ::core::marker::Copy for WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL_TYPE {}
impl ::core::clone::Clone for WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_WINDOWS_INTEGRATED_AUTH_PACKAGE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_WINDOWS_INTEGRATED_AUTH_PACKAGE_KERBEROS: WS_WINDOWS_INTEGRATED_AUTH_PACKAGE = WS_WINDOWS_INTEGRATED_AUTH_PACKAGE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_WINDOWS_INTEGRATED_AUTH_PACKAGE_NTLM: WS_WINDOWS_INTEGRATED_AUTH_PACKAGE = WS_WINDOWS_INTEGRATED_AUTH_PACKAGE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_WINDOWS_INTEGRATED_AUTH_PACKAGE_SPNEGO: WS_WINDOWS_INTEGRATED_AUTH_PACKAGE = WS_WINDOWS_INTEGRATED_AUTH_PACKAGE(3i32);
impl ::core::marker::Copy for WS_WINDOWS_INTEGRATED_AUTH_PACKAGE {}
impl ::core::clone::Clone for WS_WINDOWS_INTEGRATED_AUTH_PACKAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_WINDOWS_INTEGRATED_AUTH_PACKAGE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_WINDOWS_INTEGRATED_AUTH_PACKAGE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_WINDOWS_INTEGRATED_AUTH_PACKAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_WINDOWS_INTEGRATED_AUTH_PACKAGE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_WRITE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(callbackstate: *const ::core::ffi::c_void, buffers: *const WS_BYTES, count: u32, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_WRITE_MESSAGE_END_CALLBACK = ::core::option::Option<unsafe extern "system" fn(channelinstance: *const ::core::ffi::c_void, message: *const WS_MESSAGE, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_WRITE_MESSAGE_START_CALLBACK = ::core::option::Option<unsafe extern "system" fn(channelinstance: *const ::core::ffi::c_void, message: *const WS_MESSAGE, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_WRITE_OPTION(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_WRITE_REQUIRED_VALUE: WS_WRITE_OPTION = WS_WRITE_OPTION(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_WRITE_REQUIRED_POINTER: WS_WRITE_OPTION = WS_WRITE_OPTION(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_WRITE_NILLABLE_VALUE: WS_WRITE_OPTION = WS_WRITE_OPTION(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_WRITE_NILLABLE_POINTER: WS_WRITE_OPTION = WS_WRITE_OPTION(4i32);
impl ::core::marker::Copy for WS_WRITE_OPTION {}
impl ::core::clone::Clone for WS_WRITE_OPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_WRITE_OPTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_WRITE_OPTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_WRITE_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_WRITE_OPTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_WRITE_TYPE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(writer: *const WS_XML_WRITER, typemapping: WS_TYPE_MAPPING, descriptiondata: *const ::core::ffi::c_void, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_WSZ_DESCRIPTION {
    pub minCharCount: u32,
    pub maxCharCount: u32,
}
impl ::core::marker::Copy for WS_WSZ_DESCRIPTION {}
impl ::core::clone::Clone for WS_WSZ_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_WSZ_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_WSZ_DESCRIPTION").field("minCharCount", &self.minCharCount).field("maxCharCount", &self.maxCharCount).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_WSZ_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_WSZ_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_WSZ_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_WSZ_DESCRIPTION {}
impl ::core::default::Default for WS_WSZ_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_XML_ATTRIBUTE {
    pub singleQuote: u8,
    pub isXmlNs: u8,
    pub prefix: *mut WS_XML_STRING,
    pub localName: *mut WS_XML_STRING,
    pub ns: *mut WS_XML_STRING,
    pub value: *mut WS_XML_TEXT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_XML_ATTRIBUTE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_XML_ATTRIBUTE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_XML_ATTRIBUTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_ATTRIBUTE").field("singleQuote", &self.singleQuote).field("isXmlNs", &self.isXmlNs).field("prefix", &self.prefix).field("localName", &self.localName).field("ns", &self.ns).field("value", &self.value).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_XML_ATTRIBUTE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_XML_ATTRIBUTE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_ATTRIBUTE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_XML_ATTRIBUTE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_XML_ATTRIBUTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_BASE64_TEXT {
    pub text: WS_XML_TEXT,
    pub bytes: *mut u8,
    pub length: u32,
}
impl ::core::marker::Copy for WS_XML_BASE64_TEXT {}
impl ::core::clone::Clone for WS_XML_BASE64_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_BASE64_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_BASE64_TEXT").field("text", &self.text).field("bytes", &self.bytes).field("length", &self.length).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_XML_BASE64_TEXT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_XML_BASE64_TEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_BASE64_TEXT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_XML_BASE64_TEXT {}
impl ::core::default::Default for WS_XML_BASE64_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_XML_BOOL_TEXT {
    pub text: WS_XML_TEXT,
    pub value: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_XML_BOOL_TEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_XML_BOOL_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_XML_BOOL_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_BOOL_TEXT").field("text", &self.text).field("value", &self.value).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_XML_BOOL_TEXT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_XML_BOOL_TEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_BOOL_TEXT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_XML_BOOL_TEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_XML_BOOL_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WS_XML_BUFFER(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_BUFFER_PROPERTY {
    pub id: WS_XML_BUFFER_PROPERTY_ID,
    pub value: *mut ::core::ffi::c_void,
    pub valueSize: u32,
}
impl ::core::marker::Copy for WS_XML_BUFFER_PROPERTY {}
impl ::core::clone::Clone for WS_XML_BUFFER_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_BUFFER_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_BUFFER_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_XML_BUFFER_PROPERTY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_XML_BUFFER_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_BUFFER_PROPERTY>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_XML_BUFFER_PROPERTY {}
impl ::core::default::Default for WS_XML_BUFFER_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_XML_BUFFER_PROPERTY_ID(pub i32);
impl ::core::marker::Copy for WS_XML_BUFFER_PROPERTY_ID {}
impl ::core::clone::Clone for WS_XML_BUFFER_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_XML_BUFFER_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_XML_BUFFER_PROPERTY_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_XML_BUFFER_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_XML_BUFFER_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_XML_CANONICALIZATION_ALGORITHM(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_EXCLUSIVE_XML_CANONICALIZATION_ALGORITHM: WS_XML_CANONICALIZATION_ALGORITHM = WS_XML_CANONICALIZATION_ALGORITHM(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_EXCLUSIVE_WITH_COMMENTS_XML_CANONICALIZATION_ALGORITHM: WS_XML_CANONICALIZATION_ALGORITHM = WS_XML_CANONICALIZATION_ALGORITHM(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_INCLUSIVE_XML_CANONICALIZATION_ALGORITHM: WS_XML_CANONICALIZATION_ALGORITHM = WS_XML_CANONICALIZATION_ALGORITHM(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_INCLUSIVE_WITH_COMMENTS_XML_CANONICALIZATION_ALGORITHM: WS_XML_CANONICALIZATION_ALGORITHM = WS_XML_CANONICALIZATION_ALGORITHM(3i32);
impl ::core::marker::Copy for WS_XML_CANONICALIZATION_ALGORITHM {}
impl ::core::clone::Clone for WS_XML_CANONICALIZATION_ALGORITHM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_XML_CANONICALIZATION_ALGORITHM {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_XML_CANONICALIZATION_ALGORITHM {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_XML_CANONICALIZATION_ALGORITHM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_XML_CANONICALIZATION_ALGORITHM").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_XML_CANONICALIZATION_INCLUSIVE_PREFIXES {
    pub prefixCount: u32,
    pub prefixes: *mut WS_XML_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_XML_CANONICALIZATION_INCLUSIVE_PREFIXES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_XML_CANONICALIZATION_INCLUSIVE_PREFIXES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_XML_CANONICALIZATION_INCLUSIVE_PREFIXES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_CANONICALIZATION_INCLUSIVE_PREFIXES").field("prefixCount", &self.prefixCount).field("prefixes", &self.prefixes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_XML_CANONICALIZATION_INCLUSIVE_PREFIXES {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_XML_CANONICALIZATION_INCLUSIVE_PREFIXES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_CANONICALIZATION_INCLUSIVE_PREFIXES>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_XML_CANONICALIZATION_INCLUSIVE_PREFIXES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_XML_CANONICALIZATION_INCLUSIVE_PREFIXES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_CANONICALIZATION_PROPERTY {
    pub id: WS_XML_CANONICALIZATION_PROPERTY_ID,
    pub value: *mut ::core::ffi::c_void,
    pub valueSize: u32,
}
impl ::core::marker::Copy for WS_XML_CANONICALIZATION_PROPERTY {}
impl ::core::clone::Clone for WS_XML_CANONICALIZATION_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_CANONICALIZATION_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_CANONICALIZATION_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_XML_CANONICALIZATION_PROPERTY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_XML_CANONICALIZATION_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_CANONICALIZATION_PROPERTY>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_XML_CANONICALIZATION_PROPERTY {}
impl ::core::default::Default for WS_XML_CANONICALIZATION_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_XML_CANONICALIZATION_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_CANONICALIZATION_PROPERTY_ALGORITHM: WS_XML_CANONICALIZATION_PROPERTY_ID = WS_XML_CANONICALIZATION_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_CANONICALIZATION_PROPERTY_INCLUSIVE_PREFIXES: WS_XML_CANONICALIZATION_PROPERTY_ID = WS_XML_CANONICALIZATION_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_CANONICALIZATION_PROPERTY_OMITTED_ELEMENT: WS_XML_CANONICALIZATION_PROPERTY_ID = WS_XML_CANONICALIZATION_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_CANONICALIZATION_PROPERTY_OUTPUT_BUFFER_SIZE: WS_XML_CANONICALIZATION_PROPERTY_ID = WS_XML_CANONICALIZATION_PROPERTY_ID(3i32);
impl ::core::marker::Copy for WS_XML_CANONICALIZATION_PROPERTY_ID {}
impl ::core::clone::Clone for WS_XML_CANONICALIZATION_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_XML_CANONICALIZATION_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_XML_CANONICALIZATION_PROPERTY_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_XML_CANONICALIZATION_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_XML_CANONICALIZATION_PROPERTY_ID").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_XML_COMMENT_NODE {
    pub node: WS_XML_NODE,
    pub value: WS_XML_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_XML_COMMENT_NODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_XML_COMMENT_NODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_XML_COMMENT_NODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_COMMENT_NODE").field("node", &self.node).field("value", &self.value).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_XML_COMMENT_NODE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_XML_COMMENT_NODE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_COMMENT_NODE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_XML_COMMENT_NODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_XML_COMMENT_NODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_DATETIME_TEXT {
    pub text: WS_XML_TEXT,
    pub value: WS_DATETIME,
}
impl ::core::marker::Copy for WS_XML_DATETIME_TEXT {}
impl ::core::clone::Clone for WS_XML_DATETIME_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_DATETIME_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_DATETIME_TEXT").field("text", &self.text).field("value", &self.value).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_XML_DATETIME_TEXT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_XML_DATETIME_TEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_DATETIME_TEXT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_XML_DATETIME_TEXT {}
impl ::core::default::Default for WS_XML_DATETIME_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_XML_DECIMAL_TEXT {
    pub text: WS_XML_TEXT,
    pub value: super::super::Foundation::DECIMAL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_XML_DECIMAL_TEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_XML_DECIMAL_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_XML_DECIMAL_TEXT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_XML_DECIMAL_TEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_DECIMAL_TEXT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_XML_DECIMAL_TEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_XML_DECIMAL_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_XML_DICTIONARY {
    pub guid: ::windows::core::GUID,
    pub strings: *mut WS_XML_STRING,
    pub stringCount: u32,
    pub isConst: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_XML_DICTIONARY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_XML_DICTIONARY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_XML_DICTIONARY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_DICTIONARY").field("guid", &self.guid).field("strings", &self.strings).field("stringCount", &self.stringCount).field("isConst", &self.isConst).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_XML_DICTIONARY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_XML_DICTIONARY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_DICTIONARY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_XML_DICTIONARY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_XML_DICTIONARY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_DOUBLE_TEXT {
    pub text: WS_XML_TEXT,
    pub value: f64,
}
impl ::core::marker::Copy for WS_XML_DOUBLE_TEXT {}
impl ::core::clone::Clone for WS_XML_DOUBLE_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_DOUBLE_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_DOUBLE_TEXT").field("text", &self.text).field("value", &self.value).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_XML_DOUBLE_TEXT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_XML_DOUBLE_TEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_DOUBLE_TEXT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_XML_DOUBLE_TEXT {}
impl ::core::default::Default for WS_XML_DOUBLE_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_XML_ELEMENT_NODE {
    pub node: WS_XML_NODE,
    pub prefix: *mut WS_XML_STRING,
    pub localName: *mut WS_XML_STRING,
    pub ns: *mut WS_XML_STRING,
    pub attributeCount: u32,
    pub attributes: *mut *mut WS_XML_ATTRIBUTE,
    pub isEmpty: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_XML_ELEMENT_NODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_XML_ELEMENT_NODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_XML_ELEMENT_NODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_ELEMENT_NODE").field("node", &self.node).field("prefix", &self.prefix).field("localName", &self.localName).field("ns", &self.ns).field("attributeCount", &self.attributeCount).field("attributes", &self.attributes).field("isEmpty", &self.isEmpty).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_XML_ELEMENT_NODE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_XML_ELEMENT_NODE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_ELEMENT_NODE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_XML_ELEMENT_NODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_XML_ELEMENT_NODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_FLOAT_TEXT {
    pub text: WS_XML_TEXT,
    pub value: f32,
}
impl ::core::marker::Copy for WS_XML_FLOAT_TEXT {}
impl ::core::clone::Clone for WS_XML_FLOAT_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_FLOAT_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_FLOAT_TEXT").field("text", &self.text).field("value", &self.value).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_XML_FLOAT_TEXT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_XML_FLOAT_TEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_FLOAT_TEXT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_XML_FLOAT_TEXT {}
impl ::core::default::Default for WS_XML_FLOAT_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_GUID_TEXT {
    pub text: WS_XML_TEXT,
    pub value: ::windows::core::GUID,
}
impl ::core::marker::Copy for WS_XML_GUID_TEXT {}
impl ::core::clone::Clone for WS_XML_GUID_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_GUID_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_GUID_TEXT").field("text", &self.text).field("value", &self.value).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_XML_GUID_TEXT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_XML_GUID_TEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_GUID_TEXT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_XML_GUID_TEXT {}
impl ::core::default::Default for WS_XML_GUID_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_INT32_TEXT {
    pub text: WS_XML_TEXT,
    pub value: i32,
}
impl ::core::marker::Copy for WS_XML_INT32_TEXT {}
impl ::core::clone::Clone for WS_XML_INT32_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_INT32_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_INT32_TEXT").field("text", &self.text).field("value", &self.value).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_XML_INT32_TEXT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_XML_INT32_TEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_INT32_TEXT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_XML_INT32_TEXT {}
impl ::core::default::Default for WS_XML_INT32_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_INT64_TEXT {
    pub text: WS_XML_TEXT,
    pub value: i64,
}
impl ::core::marker::Copy for WS_XML_INT64_TEXT {}
impl ::core::clone::Clone for WS_XML_INT64_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_INT64_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_INT64_TEXT").field("text", &self.text).field("value", &self.value).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_XML_INT64_TEXT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_XML_INT64_TEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_INT64_TEXT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_XML_INT64_TEXT {}
impl ::core::default::Default for WS_XML_INT64_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_LIST_TEXT {
    pub text: WS_XML_TEXT,
    pub itemCount: u32,
    pub items: *mut *mut WS_XML_TEXT,
}
impl ::core::marker::Copy for WS_XML_LIST_TEXT {}
impl ::core::clone::Clone for WS_XML_LIST_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_LIST_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_LIST_TEXT").field("text", &self.text).field("itemCount", &self.itemCount).field("items", &self.items).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_XML_LIST_TEXT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_XML_LIST_TEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_LIST_TEXT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_XML_LIST_TEXT {}
impl ::core::default::Default for WS_XML_LIST_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_NODE {
    pub nodeType: WS_XML_NODE_TYPE,
}
impl ::core::marker::Copy for WS_XML_NODE {}
impl ::core::clone::Clone for WS_XML_NODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_NODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_NODE").field("nodeType", &self.nodeType).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_XML_NODE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_XML_NODE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_NODE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_XML_NODE {}
impl ::core::default::Default for WS_XML_NODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_NODE_POSITION {
    pub buffer: *mut WS_XML_BUFFER,
    pub node: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WS_XML_NODE_POSITION {}
impl ::core::clone::Clone for WS_XML_NODE_POSITION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_NODE_POSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_NODE_POSITION").field("buffer", &self.buffer).field("node", &self.node).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_XML_NODE_POSITION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_XML_NODE_POSITION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_NODE_POSITION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_XML_NODE_POSITION {}
impl ::core::default::Default for WS_XML_NODE_POSITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_XML_NODE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_NODE_TYPE_ELEMENT: WS_XML_NODE_TYPE = WS_XML_NODE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_NODE_TYPE_TEXT: WS_XML_NODE_TYPE = WS_XML_NODE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_NODE_TYPE_END_ELEMENT: WS_XML_NODE_TYPE = WS_XML_NODE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_NODE_TYPE_COMMENT: WS_XML_NODE_TYPE = WS_XML_NODE_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_NODE_TYPE_CDATA: WS_XML_NODE_TYPE = WS_XML_NODE_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_NODE_TYPE_END_CDATA: WS_XML_NODE_TYPE = WS_XML_NODE_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_NODE_TYPE_EOF: WS_XML_NODE_TYPE = WS_XML_NODE_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_NODE_TYPE_BOF: WS_XML_NODE_TYPE = WS_XML_NODE_TYPE(9i32);
impl ::core::marker::Copy for WS_XML_NODE_TYPE {}
impl ::core::clone::Clone for WS_XML_NODE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_XML_NODE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_XML_NODE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_XML_NODE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_XML_NODE_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_XML_QNAME {
    pub localName: WS_XML_STRING,
    pub ns: WS_XML_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_XML_QNAME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_XML_QNAME {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_XML_QNAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_QNAME").field("localName", &self.localName).field("ns", &self.ns).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_XML_QNAME {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_XML_QNAME {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_QNAME>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_XML_QNAME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_XML_QNAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_QNAME_DESCRIPTION {
    pub minLocalNameByteCount: u32,
    pub maxLocalNameByteCount: u32,
    pub minNsByteCount: u32,
    pub maxNsByteCount: u32,
}
impl ::core::marker::Copy for WS_XML_QNAME_DESCRIPTION {}
impl ::core::clone::Clone for WS_XML_QNAME_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_QNAME_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_QNAME_DESCRIPTION").field("minLocalNameByteCount", &self.minLocalNameByteCount).field("maxLocalNameByteCount", &self.maxLocalNameByteCount).field("minNsByteCount", &self.minNsByteCount).field("maxNsByteCount", &self.maxNsByteCount).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_XML_QNAME_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_XML_QNAME_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_QNAME_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_XML_QNAME_DESCRIPTION {}
impl ::core::default::Default for WS_XML_QNAME_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_XML_QNAME_TEXT {
    pub text: WS_XML_TEXT,
    pub prefix: *mut WS_XML_STRING,
    pub localName: *mut WS_XML_STRING,
    pub ns: *mut WS_XML_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_XML_QNAME_TEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_XML_QNAME_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_XML_QNAME_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_QNAME_TEXT").field("text", &self.text).field("prefix", &self.prefix).field("localName", &self.localName).field("ns", &self.ns).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_XML_QNAME_TEXT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_XML_QNAME_TEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_QNAME_TEXT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_XML_QNAME_TEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_XML_QNAME_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WS_XML_READER(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_XML_READER_BINARY_ENCODING {
    pub encoding: WS_XML_READER_ENCODING,
    pub staticDictionary: *mut WS_XML_DICTIONARY,
    pub dynamicDictionary: *mut WS_XML_DICTIONARY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_XML_READER_BINARY_ENCODING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_XML_READER_BINARY_ENCODING {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_XML_READER_BINARY_ENCODING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_READER_BINARY_ENCODING").field("encoding", &self.encoding).field("staticDictionary", &self.staticDictionary).field("dynamicDictionary", &self.dynamicDictionary).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_XML_READER_BINARY_ENCODING {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_XML_READER_BINARY_ENCODING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_READER_BINARY_ENCODING>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_XML_READER_BINARY_ENCODING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_XML_READER_BINARY_ENCODING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_READER_BUFFER_INPUT {
    pub input: WS_XML_READER_INPUT,
    pub encodedData: *mut ::core::ffi::c_void,
    pub encodedDataSize: u32,
}
impl ::core::marker::Copy for WS_XML_READER_BUFFER_INPUT {}
impl ::core::clone::Clone for WS_XML_READER_BUFFER_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_READER_BUFFER_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_READER_BUFFER_INPUT").field("input", &self.input).field("encodedData", &self.encodedData).field("encodedDataSize", &self.encodedDataSize).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_XML_READER_BUFFER_INPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_XML_READER_BUFFER_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_READER_BUFFER_INPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_XML_READER_BUFFER_INPUT {}
impl ::core::default::Default for WS_XML_READER_BUFFER_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_READER_ENCODING {
    pub encodingType: WS_XML_READER_ENCODING_TYPE,
}
impl ::core::marker::Copy for WS_XML_READER_ENCODING {}
impl ::core::clone::Clone for WS_XML_READER_ENCODING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_READER_ENCODING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_READER_ENCODING").field("encodingType", &self.encodingType).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_XML_READER_ENCODING {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_XML_READER_ENCODING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_READER_ENCODING>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_XML_READER_ENCODING {}
impl ::core::default::Default for WS_XML_READER_ENCODING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_XML_READER_ENCODING_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_READER_ENCODING_TYPE_TEXT: WS_XML_READER_ENCODING_TYPE = WS_XML_READER_ENCODING_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_READER_ENCODING_TYPE_BINARY: WS_XML_READER_ENCODING_TYPE = WS_XML_READER_ENCODING_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_READER_ENCODING_TYPE_MTOM: WS_XML_READER_ENCODING_TYPE = WS_XML_READER_ENCODING_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_READER_ENCODING_TYPE_RAW: WS_XML_READER_ENCODING_TYPE = WS_XML_READER_ENCODING_TYPE(4i32);
impl ::core::marker::Copy for WS_XML_READER_ENCODING_TYPE {}
impl ::core::clone::Clone for WS_XML_READER_ENCODING_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_XML_READER_ENCODING_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_XML_READER_ENCODING_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_XML_READER_ENCODING_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_XML_READER_ENCODING_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_READER_INPUT {
    pub inputType: WS_XML_READER_INPUT_TYPE,
}
impl ::core::marker::Copy for WS_XML_READER_INPUT {}
impl ::core::clone::Clone for WS_XML_READER_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_READER_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_READER_INPUT").field("inputType", &self.inputType).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_XML_READER_INPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_XML_READER_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_READER_INPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_XML_READER_INPUT {}
impl ::core::default::Default for WS_XML_READER_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_XML_READER_INPUT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_READER_INPUT_TYPE_BUFFER: WS_XML_READER_INPUT_TYPE = WS_XML_READER_INPUT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_READER_INPUT_TYPE_STREAM: WS_XML_READER_INPUT_TYPE = WS_XML_READER_INPUT_TYPE(2i32);
impl ::core::marker::Copy for WS_XML_READER_INPUT_TYPE {}
impl ::core::clone::Clone for WS_XML_READER_INPUT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_XML_READER_INPUT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_XML_READER_INPUT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_XML_READER_INPUT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_XML_READER_INPUT_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_XML_READER_MTOM_ENCODING {
    pub encoding: WS_XML_READER_ENCODING,
    pub textEncoding: *mut WS_XML_READER_ENCODING,
    pub readMimeHeader: super::super::Foundation::BOOL,
    pub startInfo: WS_STRING,
    pub boundary: WS_STRING,
    pub startUri: WS_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_XML_READER_MTOM_ENCODING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_XML_READER_MTOM_ENCODING {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_XML_READER_MTOM_ENCODING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_READER_MTOM_ENCODING").field("encoding", &self.encoding).field("textEncoding", &self.textEncoding).field("readMimeHeader", &self.readMimeHeader).field("startInfo", &self.startInfo).field("boundary", &self.boundary).field("startUri", &self.startUri).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_XML_READER_MTOM_ENCODING {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_XML_READER_MTOM_ENCODING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_READER_MTOM_ENCODING>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_XML_READER_MTOM_ENCODING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_XML_READER_MTOM_ENCODING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_READER_PROPERTIES {
    pub properties: *mut WS_XML_READER_PROPERTY,
    pub propertyCount: u32,
}
impl ::core::marker::Copy for WS_XML_READER_PROPERTIES {}
impl ::core::clone::Clone for WS_XML_READER_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_READER_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_READER_PROPERTIES").field("properties", &self.properties).field("propertyCount", &self.propertyCount).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_XML_READER_PROPERTIES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_XML_READER_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_READER_PROPERTIES>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_XML_READER_PROPERTIES {}
impl ::core::default::Default for WS_XML_READER_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_READER_PROPERTY {
    pub id: WS_XML_READER_PROPERTY_ID,
    pub value: *mut ::core::ffi::c_void,
    pub valueSize: u32,
}
impl ::core::marker::Copy for WS_XML_READER_PROPERTY {}
impl ::core::clone::Clone for WS_XML_READER_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_READER_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_READER_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_XML_READER_PROPERTY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_XML_READER_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_READER_PROPERTY>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_XML_READER_PROPERTY {}
impl ::core::default::Default for WS_XML_READER_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_XML_READER_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_READER_PROPERTY_MAX_DEPTH: WS_XML_READER_PROPERTY_ID = WS_XML_READER_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_READER_PROPERTY_ALLOW_FRAGMENT: WS_XML_READER_PROPERTY_ID = WS_XML_READER_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_READER_PROPERTY_MAX_ATTRIBUTES: WS_XML_READER_PROPERTY_ID = WS_XML_READER_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_READER_PROPERTY_READ_DECLARATION: WS_XML_READER_PROPERTY_ID = WS_XML_READER_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_READER_PROPERTY_CHARSET: WS_XML_READER_PROPERTY_ID = WS_XML_READER_PROPERTY_ID(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_READER_PROPERTY_ROW: WS_XML_READER_PROPERTY_ID = WS_XML_READER_PROPERTY_ID(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_READER_PROPERTY_COLUMN: WS_XML_READER_PROPERTY_ID = WS_XML_READER_PROPERTY_ID(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_READER_PROPERTY_UTF8_TRIM_SIZE: WS_XML_READER_PROPERTY_ID = WS_XML_READER_PROPERTY_ID(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_READER_PROPERTY_STREAM_BUFFER_SIZE: WS_XML_READER_PROPERTY_ID = WS_XML_READER_PROPERTY_ID(8i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_READER_PROPERTY_IN_ATTRIBUTE: WS_XML_READER_PROPERTY_ID = WS_XML_READER_PROPERTY_ID(9i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_READER_PROPERTY_STREAM_MAX_ROOT_MIME_PART_SIZE: WS_XML_READER_PROPERTY_ID = WS_XML_READER_PROPERTY_ID(10i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_READER_PROPERTY_STREAM_MAX_MIME_HEADERS_SIZE: WS_XML_READER_PROPERTY_ID = WS_XML_READER_PROPERTY_ID(11i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_READER_PROPERTY_MAX_MIME_PARTS: WS_XML_READER_PROPERTY_ID = WS_XML_READER_PROPERTY_ID(12i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_READER_PROPERTY_ALLOW_INVALID_CHARACTER_REFERENCES: WS_XML_READER_PROPERTY_ID = WS_XML_READER_PROPERTY_ID(13i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_READER_PROPERTY_MAX_NAMESPACES: WS_XML_READER_PROPERTY_ID = WS_XML_READER_PROPERTY_ID(14i32);
impl ::core::marker::Copy for WS_XML_READER_PROPERTY_ID {}
impl ::core::clone::Clone for WS_XML_READER_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_XML_READER_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_XML_READER_PROPERTY_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_XML_READER_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_XML_READER_PROPERTY_ID").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_READER_RAW_ENCODING {
    pub encoding: WS_XML_READER_ENCODING,
}
impl ::core::marker::Copy for WS_XML_READER_RAW_ENCODING {}
impl ::core::clone::Clone for WS_XML_READER_RAW_ENCODING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_READER_RAW_ENCODING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_READER_RAW_ENCODING").field("encoding", &self.encoding).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_XML_READER_RAW_ENCODING {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_XML_READER_RAW_ENCODING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_READER_RAW_ENCODING>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_XML_READER_RAW_ENCODING {}
impl ::core::default::Default for WS_XML_READER_RAW_ENCODING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_READER_STREAM_INPUT {
    pub input: WS_XML_READER_INPUT,
    pub readCallback: WS_READ_CALLBACK,
    pub readCallbackState: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WS_XML_READER_STREAM_INPUT {}
impl ::core::clone::Clone for WS_XML_READER_STREAM_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_READER_STREAM_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_READER_STREAM_INPUT").field("input", &self.input).field("readCallback", &self.readCallback.map(|f| f as usize)).field("readCallbackState", &self.readCallbackState).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_XML_READER_STREAM_INPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_XML_READER_STREAM_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_READER_STREAM_INPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_XML_READER_STREAM_INPUT {}
impl ::core::default::Default for WS_XML_READER_STREAM_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_READER_TEXT_ENCODING {
    pub encoding: WS_XML_READER_ENCODING,
    pub charSet: WS_CHARSET,
}
impl ::core::marker::Copy for WS_XML_READER_TEXT_ENCODING {}
impl ::core::clone::Clone for WS_XML_READER_TEXT_ENCODING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_READER_TEXT_ENCODING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_READER_TEXT_ENCODING").field("encoding", &self.encoding).field("charSet", &self.charSet).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_XML_READER_TEXT_ENCODING {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_XML_READER_TEXT_ENCODING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_READER_TEXT_ENCODING>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_XML_READER_TEXT_ENCODING {}
impl ::core::default::Default for WS_XML_READER_TEXT_ENCODING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_SECURITY_TOKEN_PROPERTY {
    pub id: WS_XML_SECURITY_TOKEN_PROPERTY_ID,
    pub value: *mut ::core::ffi::c_void,
    pub valueSize: u32,
}
impl ::core::marker::Copy for WS_XML_SECURITY_TOKEN_PROPERTY {}
impl ::core::clone::Clone for WS_XML_SECURITY_TOKEN_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_SECURITY_TOKEN_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_SECURITY_TOKEN_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_XML_SECURITY_TOKEN_PROPERTY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_XML_SECURITY_TOKEN_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_SECURITY_TOKEN_PROPERTY>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_XML_SECURITY_TOKEN_PROPERTY {}
impl ::core::default::Default for WS_XML_SECURITY_TOKEN_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_XML_SECURITY_TOKEN_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_SECURITY_TOKEN_PROPERTY_ATTACHED_REFERENCE: WS_XML_SECURITY_TOKEN_PROPERTY_ID = WS_XML_SECURITY_TOKEN_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_SECURITY_TOKEN_PROPERTY_UNATTACHED_REFERENCE: WS_XML_SECURITY_TOKEN_PROPERTY_ID = WS_XML_SECURITY_TOKEN_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_SECURITY_TOKEN_PROPERTY_VALID_FROM_TIME: WS_XML_SECURITY_TOKEN_PROPERTY_ID = WS_XML_SECURITY_TOKEN_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_SECURITY_TOKEN_PROPERTY_VALID_TILL_TIME: WS_XML_SECURITY_TOKEN_PROPERTY_ID = WS_XML_SECURITY_TOKEN_PROPERTY_ID(4i32);
impl ::core::marker::Copy for WS_XML_SECURITY_TOKEN_PROPERTY_ID {}
impl ::core::clone::Clone for WS_XML_SECURITY_TOKEN_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_XML_SECURITY_TOKEN_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_XML_SECURITY_TOKEN_PROPERTY_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_XML_SECURITY_TOKEN_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_XML_SECURITY_TOKEN_PROPERTY_ID").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_XML_STRING {
    pub length: u32,
    pub bytes: *mut u8,
    pub dictionary: *mut WS_XML_DICTIONARY,
    pub id: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_XML_STRING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_XML_STRING {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_XML_STRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_STRING").field("length", &self.length).field("bytes", &self.bytes).field("dictionary", &self.dictionary).field("id", &self.id).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_XML_STRING {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_XML_STRING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_STRING>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_XML_STRING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_XML_STRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_STRING_DESCRIPTION {
    pub minByteCount: u32,
    pub maxByteCount: u32,
}
impl ::core::marker::Copy for WS_XML_STRING_DESCRIPTION {}
impl ::core::clone::Clone for WS_XML_STRING_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_STRING_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_STRING_DESCRIPTION").field("minByteCount", &self.minByteCount).field("maxByteCount", &self.maxByteCount).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_XML_STRING_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_XML_STRING_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_STRING_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_XML_STRING_DESCRIPTION {}
impl ::core::default::Default for WS_XML_STRING_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_TEXT {
    pub textType: WS_XML_TEXT_TYPE,
}
impl ::core::marker::Copy for WS_XML_TEXT {}
impl ::core::clone::Clone for WS_XML_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_TEXT").field("textType", &self.textType).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_XML_TEXT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_XML_TEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_TEXT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_XML_TEXT {}
impl ::core::default::Default for WS_XML_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_TEXT_NODE {
    pub node: WS_XML_NODE,
    pub text: *mut WS_XML_TEXT,
}
impl ::core::marker::Copy for WS_XML_TEXT_NODE {}
impl ::core::clone::Clone for WS_XML_TEXT_NODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_TEXT_NODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_TEXT_NODE").field("node", &self.node).field("text", &self.text).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_XML_TEXT_NODE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_XML_TEXT_NODE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_TEXT_NODE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_XML_TEXT_NODE {}
impl ::core::default::Default for WS_XML_TEXT_NODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_XML_TEXT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_TEXT_TYPE_UTF8: WS_XML_TEXT_TYPE = WS_XML_TEXT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_TEXT_TYPE_UTF16: WS_XML_TEXT_TYPE = WS_XML_TEXT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_TEXT_TYPE_BASE64: WS_XML_TEXT_TYPE = WS_XML_TEXT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_TEXT_TYPE_BOOL: WS_XML_TEXT_TYPE = WS_XML_TEXT_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_TEXT_TYPE_INT32: WS_XML_TEXT_TYPE = WS_XML_TEXT_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_TEXT_TYPE_INT64: WS_XML_TEXT_TYPE = WS_XML_TEXT_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_TEXT_TYPE_UINT64: WS_XML_TEXT_TYPE = WS_XML_TEXT_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_TEXT_TYPE_FLOAT: WS_XML_TEXT_TYPE = WS_XML_TEXT_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_TEXT_TYPE_DOUBLE: WS_XML_TEXT_TYPE = WS_XML_TEXT_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_TEXT_TYPE_DECIMAL: WS_XML_TEXT_TYPE = WS_XML_TEXT_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_TEXT_TYPE_GUID: WS_XML_TEXT_TYPE = WS_XML_TEXT_TYPE(11i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_TEXT_TYPE_UNIQUE_ID: WS_XML_TEXT_TYPE = WS_XML_TEXT_TYPE(12i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_TEXT_TYPE_DATETIME: WS_XML_TEXT_TYPE = WS_XML_TEXT_TYPE(13i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_TEXT_TYPE_TIMESPAN: WS_XML_TEXT_TYPE = WS_XML_TEXT_TYPE(14i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_TEXT_TYPE_QNAME: WS_XML_TEXT_TYPE = WS_XML_TEXT_TYPE(15i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_TEXT_TYPE_LIST: WS_XML_TEXT_TYPE = WS_XML_TEXT_TYPE(16i32);
impl ::core::marker::Copy for WS_XML_TEXT_TYPE {}
impl ::core::clone::Clone for WS_XML_TEXT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_XML_TEXT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_XML_TEXT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_XML_TEXT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_XML_TEXT_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_TIMESPAN_TEXT {
    pub text: WS_XML_TEXT,
    pub value: WS_TIMESPAN,
}
impl ::core::marker::Copy for WS_XML_TIMESPAN_TEXT {}
impl ::core::clone::Clone for WS_XML_TIMESPAN_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_TIMESPAN_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_TIMESPAN_TEXT").field("text", &self.text).field("value", &self.value).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_XML_TIMESPAN_TEXT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_XML_TIMESPAN_TEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_TIMESPAN_TEXT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_XML_TIMESPAN_TEXT {}
impl ::core::default::Default for WS_XML_TIMESPAN_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_TOKEN_MESSAGE_SECURITY_BINDING {
    pub binding: WS_SECURITY_BINDING,
    pub bindingUsage: WS_MESSAGE_SECURITY_USAGE,
    pub xmlToken: *mut WS_SECURITY_TOKEN,
}
impl ::core::marker::Copy for WS_XML_TOKEN_MESSAGE_SECURITY_BINDING {}
impl ::core::clone::Clone for WS_XML_TOKEN_MESSAGE_SECURITY_BINDING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_TOKEN_MESSAGE_SECURITY_BINDING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_TOKEN_MESSAGE_SECURITY_BINDING").field("binding", &self.binding).field("bindingUsage", &self.bindingUsage).field("xmlToken", &self.xmlToken).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_XML_TOKEN_MESSAGE_SECURITY_BINDING {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_XML_TOKEN_MESSAGE_SECURITY_BINDING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_TOKEN_MESSAGE_SECURITY_BINDING>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_XML_TOKEN_MESSAGE_SECURITY_BINDING {}
impl ::core::default::Default for WS_XML_TOKEN_MESSAGE_SECURITY_BINDING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_UINT64_TEXT {
    pub text: WS_XML_TEXT,
    pub value: u64,
}
impl ::core::marker::Copy for WS_XML_UINT64_TEXT {}
impl ::core::clone::Clone for WS_XML_UINT64_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_UINT64_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_UINT64_TEXT").field("text", &self.text).field("value", &self.value).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_XML_UINT64_TEXT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_XML_UINT64_TEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_UINT64_TEXT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_XML_UINT64_TEXT {}
impl ::core::default::Default for WS_XML_UINT64_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_UNIQUE_ID_TEXT {
    pub text: WS_XML_TEXT,
    pub value: ::windows::core::GUID,
}
impl ::core::marker::Copy for WS_XML_UNIQUE_ID_TEXT {}
impl ::core::clone::Clone for WS_XML_UNIQUE_ID_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_UNIQUE_ID_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_UNIQUE_ID_TEXT").field("text", &self.text).field("value", &self.value).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_XML_UNIQUE_ID_TEXT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_XML_UNIQUE_ID_TEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_UNIQUE_ID_TEXT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_XML_UNIQUE_ID_TEXT {}
impl ::core::default::Default for WS_XML_UNIQUE_ID_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_UTF16_TEXT {
    pub text: WS_XML_TEXT,
    pub bytes: *mut u8,
    pub byteCount: u32,
}
impl ::core::marker::Copy for WS_XML_UTF16_TEXT {}
impl ::core::clone::Clone for WS_XML_UTF16_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_UTF16_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_UTF16_TEXT").field("text", &self.text).field("bytes", &self.bytes).field("byteCount", &self.byteCount).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_XML_UTF16_TEXT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_XML_UTF16_TEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_UTF16_TEXT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_XML_UTF16_TEXT {}
impl ::core::default::Default for WS_XML_UTF16_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_XML_UTF8_TEXT {
    pub text: WS_XML_TEXT,
    pub value: WS_XML_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_XML_UTF8_TEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_XML_UTF8_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_XML_UTF8_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_UTF8_TEXT").field("text", &self.text).field("value", &self.value).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_XML_UTF8_TEXT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_XML_UTF8_TEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_UTF8_TEXT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_XML_UTF8_TEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_XML_UTF8_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WS_XML_WRITER(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_XML_WRITER_BINARY_ENCODING {
    pub encoding: WS_XML_WRITER_ENCODING,
    pub staticDictionary: *mut WS_XML_DICTIONARY,
    pub dynamicStringCallback: WS_DYNAMIC_STRING_CALLBACK,
    pub dynamicStringCallbackState: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_XML_WRITER_BINARY_ENCODING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_XML_WRITER_BINARY_ENCODING {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_XML_WRITER_BINARY_ENCODING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_WRITER_BINARY_ENCODING").field("encoding", &self.encoding).field("staticDictionary", &self.staticDictionary).field("dynamicStringCallback", &self.dynamicStringCallback.map(|f| f as usize)).field("dynamicStringCallbackState", &self.dynamicStringCallbackState).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_XML_WRITER_BINARY_ENCODING {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_XML_WRITER_BINARY_ENCODING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_WRITER_BINARY_ENCODING>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_XML_WRITER_BINARY_ENCODING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_XML_WRITER_BINARY_ENCODING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_WRITER_BUFFER_OUTPUT {
    pub output: WS_XML_WRITER_OUTPUT,
}
impl ::core::marker::Copy for WS_XML_WRITER_BUFFER_OUTPUT {}
impl ::core::clone::Clone for WS_XML_WRITER_BUFFER_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_WRITER_BUFFER_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_WRITER_BUFFER_OUTPUT").field("output", &self.output).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_XML_WRITER_BUFFER_OUTPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_XML_WRITER_BUFFER_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_WRITER_BUFFER_OUTPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_XML_WRITER_BUFFER_OUTPUT {}
impl ::core::default::Default for WS_XML_WRITER_BUFFER_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_WRITER_ENCODING {
    pub encodingType: WS_XML_WRITER_ENCODING_TYPE,
}
impl ::core::marker::Copy for WS_XML_WRITER_ENCODING {}
impl ::core::clone::Clone for WS_XML_WRITER_ENCODING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_WRITER_ENCODING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_WRITER_ENCODING").field("encodingType", &self.encodingType).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_XML_WRITER_ENCODING {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_XML_WRITER_ENCODING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_WRITER_ENCODING>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_XML_WRITER_ENCODING {}
impl ::core::default::Default for WS_XML_WRITER_ENCODING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_XML_WRITER_ENCODING_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_ENCODING_TYPE_TEXT: WS_XML_WRITER_ENCODING_TYPE = WS_XML_WRITER_ENCODING_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_ENCODING_TYPE_BINARY: WS_XML_WRITER_ENCODING_TYPE = WS_XML_WRITER_ENCODING_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_ENCODING_TYPE_MTOM: WS_XML_WRITER_ENCODING_TYPE = WS_XML_WRITER_ENCODING_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_ENCODING_TYPE_RAW: WS_XML_WRITER_ENCODING_TYPE = WS_XML_WRITER_ENCODING_TYPE(4i32);
impl ::core::marker::Copy for WS_XML_WRITER_ENCODING_TYPE {}
impl ::core::clone::Clone for WS_XML_WRITER_ENCODING_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_XML_WRITER_ENCODING_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_XML_WRITER_ENCODING_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_XML_WRITER_ENCODING_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_XML_WRITER_ENCODING_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_XML_WRITER_MTOM_ENCODING {
    pub encoding: WS_XML_WRITER_ENCODING,
    pub textEncoding: *mut WS_XML_WRITER_ENCODING,
    pub writeMimeHeader: super::super::Foundation::BOOL,
    pub boundary: WS_STRING,
    pub startInfo: WS_STRING,
    pub startUri: WS_STRING,
    pub maxInlineByteCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_XML_WRITER_MTOM_ENCODING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_XML_WRITER_MTOM_ENCODING {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_XML_WRITER_MTOM_ENCODING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_WRITER_MTOM_ENCODING").field("encoding", &self.encoding).field("textEncoding", &self.textEncoding).field("writeMimeHeader", &self.writeMimeHeader).field("boundary", &self.boundary).field("startInfo", &self.startInfo).field("startUri", &self.startUri).field("maxInlineByteCount", &self.maxInlineByteCount).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WS_XML_WRITER_MTOM_ENCODING {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_XML_WRITER_MTOM_ENCODING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_WRITER_MTOM_ENCODING>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_XML_WRITER_MTOM_ENCODING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_XML_WRITER_MTOM_ENCODING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_WRITER_OUTPUT {
    pub outputType: WS_XML_WRITER_OUTPUT_TYPE,
}
impl ::core::marker::Copy for WS_XML_WRITER_OUTPUT {}
impl ::core::clone::Clone for WS_XML_WRITER_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_WRITER_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_WRITER_OUTPUT").field("outputType", &self.outputType).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_XML_WRITER_OUTPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_XML_WRITER_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_WRITER_OUTPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_XML_WRITER_OUTPUT {}
impl ::core::default::Default for WS_XML_WRITER_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_XML_WRITER_OUTPUT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_OUTPUT_TYPE_BUFFER: WS_XML_WRITER_OUTPUT_TYPE = WS_XML_WRITER_OUTPUT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_OUTPUT_TYPE_STREAM: WS_XML_WRITER_OUTPUT_TYPE = WS_XML_WRITER_OUTPUT_TYPE(2i32);
impl ::core::marker::Copy for WS_XML_WRITER_OUTPUT_TYPE {}
impl ::core::clone::Clone for WS_XML_WRITER_OUTPUT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_XML_WRITER_OUTPUT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_XML_WRITER_OUTPUT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_XML_WRITER_OUTPUT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_XML_WRITER_OUTPUT_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_WRITER_PROPERTIES {
    pub properties: *mut WS_XML_WRITER_PROPERTY,
    pub propertyCount: u32,
}
impl ::core::marker::Copy for WS_XML_WRITER_PROPERTIES {}
impl ::core::clone::Clone for WS_XML_WRITER_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_WRITER_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_WRITER_PROPERTIES").field("properties", &self.properties).field("propertyCount", &self.propertyCount).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_XML_WRITER_PROPERTIES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_XML_WRITER_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_WRITER_PROPERTIES>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_XML_WRITER_PROPERTIES {}
impl ::core::default::Default for WS_XML_WRITER_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_WRITER_PROPERTY {
    pub id: WS_XML_WRITER_PROPERTY_ID,
    pub value: *mut ::core::ffi::c_void,
    pub valueSize: u32,
}
impl ::core::marker::Copy for WS_XML_WRITER_PROPERTY {}
impl ::core::clone::Clone for WS_XML_WRITER_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_WRITER_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_WRITER_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_XML_WRITER_PROPERTY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_XML_WRITER_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_WRITER_PROPERTY>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_XML_WRITER_PROPERTY {}
impl ::core::default::Default for WS_XML_WRITER_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WS_XML_WRITER_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_PROPERTY_MAX_DEPTH: WS_XML_WRITER_PROPERTY_ID = WS_XML_WRITER_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_PROPERTY_ALLOW_FRAGMENT: WS_XML_WRITER_PROPERTY_ID = WS_XML_WRITER_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_PROPERTY_MAX_ATTRIBUTES: WS_XML_WRITER_PROPERTY_ID = WS_XML_WRITER_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_PROPERTY_WRITE_DECLARATION: WS_XML_WRITER_PROPERTY_ID = WS_XML_WRITER_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_PROPERTY_INDENT: WS_XML_WRITER_PROPERTY_ID = WS_XML_WRITER_PROPERTY_ID(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_PROPERTY_BUFFER_TRIM_SIZE: WS_XML_WRITER_PROPERTY_ID = WS_XML_WRITER_PROPERTY_ID(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_PROPERTY_CHARSET: WS_XML_WRITER_PROPERTY_ID = WS_XML_WRITER_PROPERTY_ID(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_PROPERTY_BUFFERS: WS_XML_WRITER_PROPERTY_ID = WS_XML_WRITER_PROPERTY_ID(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_PROPERTY_BUFFER_MAX_SIZE: WS_XML_WRITER_PROPERTY_ID = WS_XML_WRITER_PROPERTY_ID(8i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_PROPERTY_BYTES: WS_XML_WRITER_PROPERTY_ID = WS_XML_WRITER_PROPERTY_ID(9i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_PROPERTY_IN_ATTRIBUTE: WS_XML_WRITER_PROPERTY_ID = WS_XML_WRITER_PROPERTY_ID(10i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_PROPERTY_MAX_MIME_PARTS_BUFFER_SIZE: WS_XML_WRITER_PROPERTY_ID = WS_XML_WRITER_PROPERTY_ID(11i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_PROPERTY_INITIAL_BUFFER: WS_XML_WRITER_PROPERTY_ID = WS_XML_WRITER_PROPERTY_ID(12i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_PROPERTY_ALLOW_INVALID_CHARACTER_REFERENCES: WS_XML_WRITER_PROPERTY_ID = WS_XML_WRITER_PROPERTY_ID(13i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_PROPERTY_MAX_NAMESPACES: WS_XML_WRITER_PROPERTY_ID = WS_XML_WRITER_PROPERTY_ID(14i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_PROPERTY_BYTES_WRITTEN: WS_XML_WRITER_PROPERTY_ID = WS_XML_WRITER_PROPERTY_ID(15i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_PROPERTY_BYTES_TO_CLOSE: WS_XML_WRITER_PROPERTY_ID = WS_XML_WRITER_PROPERTY_ID(16i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_PROPERTY_COMPRESS_EMPTY_ELEMENTS: WS_XML_WRITER_PROPERTY_ID = WS_XML_WRITER_PROPERTY_ID(17i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_PROPERTY_EMIT_UNCOMPRESSED_EMPTY_ELEMENTS: WS_XML_WRITER_PROPERTY_ID = WS_XML_WRITER_PROPERTY_ID(18i32);
impl ::core::marker::Copy for WS_XML_WRITER_PROPERTY_ID {}
impl ::core::clone::Clone for WS_XML_WRITER_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_XML_WRITER_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WS_XML_WRITER_PROPERTY_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for WS_XML_WRITER_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_XML_WRITER_PROPERTY_ID").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_WRITER_RAW_ENCODING {
    pub encoding: WS_XML_WRITER_ENCODING,
}
impl ::core::marker::Copy for WS_XML_WRITER_RAW_ENCODING {}
impl ::core::clone::Clone for WS_XML_WRITER_RAW_ENCODING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_WRITER_RAW_ENCODING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_WRITER_RAW_ENCODING").field("encoding", &self.encoding).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_XML_WRITER_RAW_ENCODING {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_XML_WRITER_RAW_ENCODING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_WRITER_RAW_ENCODING>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_XML_WRITER_RAW_ENCODING {}
impl ::core::default::Default for WS_XML_WRITER_RAW_ENCODING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_WRITER_STREAM_OUTPUT {
    pub output: WS_XML_WRITER_OUTPUT,
    pub writeCallback: WS_WRITE_CALLBACK,
    pub writeCallbackState: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WS_XML_WRITER_STREAM_OUTPUT {}
impl ::core::clone::Clone for WS_XML_WRITER_STREAM_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_WRITER_STREAM_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_WRITER_STREAM_OUTPUT").field("output", &self.output).field("writeCallback", &self.writeCallback.map(|f| f as usize)).field("writeCallbackState", &self.writeCallbackState).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_XML_WRITER_STREAM_OUTPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_XML_WRITER_STREAM_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_WRITER_STREAM_OUTPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_XML_WRITER_STREAM_OUTPUT {}
impl ::core::default::Default for WS_XML_WRITER_STREAM_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_WRITER_TEXT_ENCODING {
    pub encoding: WS_XML_WRITER_ENCODING,
    pub charSet: WS_CHARSET,
}
impl ::core::marker::Copy for WS_XML_WRITER_TEXT_ENCODING {}
impl ::core::clone::Clone for WS_XML_WRITER_TEXT_ENCODING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_WRITER_TEXT_ENCODING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_WRITER_TEXT_ENCODING").field("encoding", &self.encoding).field("charSet", &self.charSet).finish()
    }
}
unsafe impl ::windows::core::Abi for WS_XML_WRITER_TEXT_ENCODING {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WS_XML_WRITER_TEXT_ENCODING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WS_XML_WRITER_TEXT_ENCODING>()) == 0 }
    }
}
impl ::core::cmp::Eq for WS_XML_WRITER_TEXT_ENCODING {}
impl ::core::default::Default for WS_XML_WRITER_TEXT_ENCODING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WebAuthNAuthenticatorGetAssertion<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hwnd: Param0, pwszrpid: Param1, pwebauthnclientdata: *const WEBAUTHN_CLIENT_DATA, pwebauthngetassertionoptions: *const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS) -> ::windows::core::Result<*mut WEBAUTHN_ASSERTION> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WebAuthNAuthenticatorGetAssertion(hwnd: super::super::Foundation::HWND, pwszrpid: ::windows::core::PCWSTR, pwebauthnclientdata: *const WEBAUTHN_CLIENT_DATA, pwebauthngetassertionoptions: *const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS, ppwebauthnassertion: *mut *mut WEBAUTHN_ASSERTION) -> ::windows::core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<*mut WEBAUTHN_ASSERTION>::zeroed();
        WebAuthNAuthenticatorGetAssertion(hwnd.into_param().abi(), pwszrpid.into_param().abi(), ::core::mem::transmute(pwebauthnclientdata), ::core::mem::transmute(pwebauthngetassertionoptions), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WEBAUTHN_ASSERTION>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WebAuthNAuthenticatorMakeCredential<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0, prpinformation: *const WEBAUTHN_RP_ENTITY_INFORMATION, puserinformation: *const WEBAUTHN_USER_ENTITY_INFORMATION, ppubkeycredparams: *const WEBAUTHN_COSE_CREDENTIAL_PARAMETERS, pwebauthnclientdata: *const WEBAUTHN_CLIENT_DATA, pwebauthnmakecredentialoptions: *const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS) -> ::windows::core::Result<*mut WEBAUTHN_CREDENTIAL_ATTESTATION> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WebAuthNAuthenticatorMakeCredential(hwnd: super::super::Foundation::HWND, prpinformation: *const WEBAUTHN_RP_ENTITY_INFORMATION, puserinformation: *const WEBAUTHN_USER_ENTITY_INFORMATION, ppubkeycredparams: *const WEBAUTHN_COSE_CREDENTIAL_PARAMETERS, pwebauthnclientdata: *const WEBAUTHN_CLIENT_DATA, pwebauthnmakecredentialoptions: *const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS, ppwebauthncredentialattestation: *mut *mut WEBAUTHN_CREDENTIAL_ATTESTATION) -> ::windows::core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<*mut WEBAUTHN_CREDENTIAL_ATTESTATION>::zeroed();
        WebAuthNAuthenticatorMakeCredential(hwnd.into_param().abi(), ::core::mem::transmute(prpinformation), ::core::mem::transmute(puserinformation), ::core::mem::transmute(ppubkeycredparams), ::core::mem::transmute(pwebauthnclientdata), ::core::mem::transmute(pwebauthnmakecredentialoptions), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WEBAUTHN_CREDENTIAL_ATTESTATION>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WebAuthNCancelCurrentOperation(pcancellationid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WebAuthNCancelCurrentOperation(pcancellationid: *const ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        WebAuthNCancelCurrentOperation(::core::mem::transmute(pcancellationid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WebAuthNFreeAssertion(pwebauthnassertion: *const WEBAUTHN_ASSERTION) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WebAuthNFreeAssertion(pwebauthnassertion: *const WEBAUTHN_ASSERTION);
        }
        WebAuthNFreeAssertion(::core::mem::transmute(pwebauthnassertion))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WebAuthNFreeCredentialAttestation(pwebauthncredentialattestation: *const WEBAUTHN_CREDENTIAL_ATTESTATION) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WebAuthNFreeCredentialAttestation(pwebauthncredentialattestation: *const WEBAUTHN_CREDENTIAL_ATTESTATION);
        }
        WebAuthNFreeCredentialAttestation(::core::mem::transmute(pwebauthncredentialattestation))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WebAuthNGetApiVersionNumber() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WebAuthNGetApiVersionNumber() -> u32;
        }
        ::core::mem::transmute(WebAuthNGetApiVersionNumber())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WebAuthNGetCancellationId() -> ::windows::core::Result<::windows::core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WebAuthNGetCancellationId(pcancellationid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows::core::GUID>::zeroed();
        WebAuthNGetCancellationId(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WebAuthNGetErrorName(hr: ::windows::core::HRESULT) -> ::windows::core::PWSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WebAuthNGetErrorName(hr: ::windows::core::HRESULT) -> ::windows::core::PWSTR;
        }
        ::core::mem::transmute(WebAuthNGetErrorName(::core::mem::transmute(hr)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WebAuthNGetW3CExceptionDOMError(hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WebAuthNGetW3CExceptionDOMError(hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT;
        }
        WebAuthNGetW3CExceptionDOMError(::core::mem::transmute(hr)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WebAuthNIsUserVerifyingPlatformAuthenticatorAvailable() -> ::windows::core::Result<super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WebAuthNIsUserVerifyingPlatformAuthenticatorAvailable(pbisuserverifyingplatformauthenticatoravailable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::BOOL>::zeroed();
        WebAuthNIsUserVerifyingPlatformAuthenticatorAvailable(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsAbandonCall(serviceproxy: *const WS_SERVICE_PROXY, callid: u32, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsAbandonCall(serviceproxy: *const WS_SERVICE_PROXY, callid: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsAbandonCall(::core::mem::transmute(serviceproxy), ::core::mem::transmute(callid), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsAbandonMessage(channel: *const WS_CHANNEL, message: *const WS_MESSAGE, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsAbandonMessage(channel: *const WS_CHANNEL, message: *const WS_MESSAGE, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsAbandonMessage(::core::mem::transmute(channel), ::core::mem::transmute(message), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsAbortChannel(channel: *const WS_CHANNEL, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsAbortChannel(channel: *const WS_CHANNEL, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsAbortChannel(::core::mem::transmute(channel), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsAbortListener(listener: *const WS_LISTENER, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsAbortListener(listener: *const WS_LISTENER, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsAbortListener(::core::mem::transmute(listener), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsAbortServiceHost(servicehost: *const WS_SERVICE_HOST, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsAbortServiceHost(servicehost: *const WS_SERVICE_HOST, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsAbortServiceHost(::core::mem::transmute(servicehost), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsAbortServiceProxy(serviceproxy: *const WS_SERVICE_PROXY, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsAbortServiceProxy(serviceproxy: *const WS_SERVICE_PROXY, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsAbortServiceProxy(::core::mem::transmute(serviceproxy), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsAcceptChannel(listener: *const WS_LISTENER, channel: *const WS_CHANNEL, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsAcceptChannel(listener: *const WS_LISTENER, channel: *const WS_CHANNEL, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsAcceptChannel(::core::mem::transmute(listener), ::core::mem::transmute(channel), ::core::mem::transmute(asynccontext), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsAddCustomHeader(message: *const WS_MESSAGE, headerdescription: *const WS_ELEMENT_DESCRIPTION, writeoption: WS_WRITE_OPTION, value: *const ::core::ffi::c_void, valuesize: u32, headerattributes: u32, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsAddCustomHeader(message: *const WS_MESSAGE, headerdescription: *const WS_ELEMENT_DESCRIPTION, writeoption: WS_WRITE_OPTION, value: *const ::core::ffi::c_void, valuesize: u32, headerattributes: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsAddCustomHeader(::core::mem::transmute(message), ::core::mem::transmute(headerdescription), ::core::mem::transmute(writeoption), ::core::mem::transmute(value), ::core::mem::transmute(valuesize), ::core::mem::transmute(headerattributes), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsAddErrorString(error: *const WS_ERROR, string: *const WS_STRING) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsAddErrorString(error: *const WS_ERROR, string: *const WS_STRING) -> ::windows::core::HRESULT;
        }
        WsAddErrorString(::core::mem::transmute(error), ::core::mem::transmute(string)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsAddMappedHeader(message: *const WS_MESSAGE, headername: *const WS_XML_STRING, valuetype: WS_TYPE, writeoption: WS_WRITE_OPTION, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsAddMappedHeader(message: *const WS_MESSAGE, headername: *const WS_XML_STRING, valuetype: WS_TYPE, writeoption: WS_WRITE_OPTION, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsAddMappedHeader(::core::mem::transmute(message), ::core::mem::transmute(headername), ::core::mem::transmute(valuetype), ::core::mem::transmute(writeoption), ::core::mem::transmute(value), ::core::mem::transmute(valuesize), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsAddressMessage(message: *const WS_MESSAGE, address: *const WS_ENDPOINT_ADDRESS, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsAddressMessage(message: *const WS_MESSAGE, address: *const WS_ENDPOINT_ADDRESS, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsAddressMessage(::core::mem::transmute(message), ::core::mem::transmute(address), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsAlloc(heap: *const WS_HEAP, size: usize, ptr: *mut *mut ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsAlloc(heap: *const WS_HEAP, size: usize, ptr: *mut *mut ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsAlloc(::core::mem::transmute(heap), ::core::mem::transmute(size), ::core::mem::transmute(ptr), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsAsyncExecute(asyncstate: *const WS_ASYNC_STATE, operation: WS_ASYNC_FUNCTION, callbackmodel: WS_CALLBACK_MODEL, callbackstate: *const ::core::ffi::c_void, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsAsyncExecute(asyncstate: *const WS_ASYNC_STATE, operation: *mut ::core::ffi::c_void, callbackmodel: WS_CALLBACK_MODEL, callbackstate: *const ::core::ffi::c_void, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsAsyncExecute(::core::mem::transmute(asyncstate), ::core::mem::transmute(operation), ::core::mem::transmute(callbackmodel), ::core::mem::transmute(callbackstate), ::core::mem::transmute(asynccontext), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsCall(serviceproxy: *const WS_SERVICE_PROXY, operation: *const WS_OPERATION_DESCRIPTION, arguments: *const *const ::core::ffi::c_void, heap: *const WS_HEAP, callproperties: &[WS_CALL_PROPERTY], asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsCall(serviceproxy: *const WS_SERVICE_PROXY, operation: *const WS_OPERATION_DESCRIPTION, arguments: *const *const ::core::ffi::c_void, heap: *const WS_HEAP, callproperties: *const WS_CALL_PROPERTY, callpropertycount: u32, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsCall(::core::mem::transmute(serviceproxy), ::core::mem::transmute(operation), ::core::mem::transmute(arguments), ::core::mem::transmute(heap), ::core::mem::transmute(::windows::core::as_ptr_or_null(callproperties)), callproperties.len() as _, ::core::mem::transmute(asynccontext), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsCheckMustUnderstandHeaders(message: *const WS_MESSAGE, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsCheckMustUnderstandHeaders(message: *const WS_MESSAGE, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsCheckMustUnderstandHeaders(::core::mem::transmute(message), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsCloseChannel(channel: *const WS_CHANNEL, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsCloseChannel(channel: *const WS_CHANNEL, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsCloseChannel(::core::mem::transmute(channel), ::core::mem::transmute(asynccontext), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsCloseListener(listener: *const WS_LISTENER, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsCloseListener(listener: *const WS_LISTENER, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsCloseListener(::core::mem::transmute(listener), ::core::mem::transmute(asynccontext), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsCloseServiceHost(servicehost: *const WS_SERVICE_HOST, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsCloseServiceHost(servicehost: *const WS_SERVICE_HOST, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsCloseServiceHost(::core::mem::transmute(servicehost), ::core::mem::transmute(asynccontext), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsCloseServiceProxy(serviceproxy: *const WS_SERVICE_PROXY, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsCloseServiceProxy(serviceproxy: *const WS_SERVICE_PROXY, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsCloseServiceProxy(::core::mem::transmute(serviceproxy), ::core::mem::transmute(asynccontext), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsCombineUrl(baseurl: *const WS_STRING, referenceurl: *const WS_STRING, flags: u32, heap: *const WS_HEAP, resulturl: *mut WS_STRING, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsCombineUrl(baseurl: *const WS_STRING, referenceurl: *const WS_STRING, flags: u32, heap: *const WS_HEAP, resulturl: *mut WS_STRING, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsCombineUrl(::core::mem::transmute(baseurl), ::core::mem::transmute(referenceurl), ::core::mem::transmute(flags), ::core::mem::transmute(heap), ::core::mem::transmute(resulturl), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsCopyError(source: *const WS_ERROR, destination: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsCopyError(source: *const WS_ERROR, destination: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsCopyError(::core::mem::transmute(source), ::core::mem::transmute(destination)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsCopyNode(writer: *const WS_XML_WRITER, reader: *const WS_XML_READER, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsCopyNode(writer: *const WS_XML_WRITER, reader: *const WS_XML_READER, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsCopyNode(::core::mem::transmute(writer), ::core::mem::transmute(reader), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsCreateChannel(channeltype: WS_CHANNEL_TYPE, channelbinding: WS_CHANNEL_BINDING, properties: &[WS_CHANNEL_PROPERTY], securitydescription: *const WS_SECURITY_DESCRIPTION, channel: *mut *mut WS_CHANNEL, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsCreateChannel(channeltype: WS_CHANNEL_TYPE, channelbinding: WS_CHANNEL_BINDING, properties: *const WS_CHANNEL_PROPERTY, propertycount: u32, securitydescription: *const WS_SECURITY_DESCRIPTION, channel: *mut *mut WS_CHANNEL, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsCreateChannel(::core::mem::transmute(channeltype), ::core::mem::transmute(channelbinding), ::core::mem::transmute(::windows::core::as_ptr_or_null(properties)), properties.len() as _, ::core::mem::transmute(securitydescription), ::core::mem::transmute(channel), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsCreateChannelForListener(listener: *const WS_LISTENER, properties: &[WS_CHANNEL_PROPERTY], channel: *mut *mut WS_CHANNEL, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsCreateChannelForListener(listener: *const WS_LISTENER, properties: *const WS_CHANNEL_PROPERTY, propertycount: u32, channel: *mut *mut WS_CHANNEL, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsCreateChannelForListener(::core::mem::transmute(listener), ::core::mem::transmute(::windows::core::as_ptr_or_null(properties)), properties.len() as _, ::core::mem::transmute(channel), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsCreateError(properties: &[WS_ERROR_PROPERTY]) -> ::windows::core::Result<*mut WS_ERROR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsCreateError(properties: *const WS_ERROR_PROPERTY, propertycount: u32, error: *mut *mut WS_ERROR) -> ::windows::core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<*mut WS_ERROR>::zeroed();
        WsCreateError(::core::mem::transmute(::windows::core::as_ptr_or_null(properties)), properties.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WS_ERROR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsCreateFaultFromError(error: *const WS_ERROR, faulterrorcode: ::windows::core::HRESULT, faultdisclosure: WS_FAULT_DISCLOSURE, heap: *const WS_HEAP) -> ::windows::core::Result<WS_FAULT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsCreateFaultFromError(error: *const WS_ERROR, faulterrorcode: ::windows::core::HRESULT, faultdisclosure: WS_FAULT_DISCLOSURE, heap: *const WS_HEAP, fault: *mut WS_FAULT) -> ::windows::core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<WS_FAULT>::zeroed();
        WsCreateFaultFromError(::core::mem::transmute(error), ::core::mem::transmute(faulterrorcode), ::core::mem::transmute(faultdisclosure), ::core::mem::transmute(heap), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WS_FAULT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsCreateHeap(maxsize: usize, trimsize: usize, properties: *const WS_HEAP_PROPERTY, propertycount: u32, heap: *mut *mut WS_HEAP, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsCreateHeap(maxsize: usize, trimsize: usize, properties: *const WS_HEAP_PROPERTY, propertycount: u32, heap: *mut *mut WS_HEAP, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsCreateHeap(::core::mem::transmute(maxsize), ::core::mem::transmute(trimsize), ::core::mem::transmute(properties), ::core::mem::transmute(propertycount), ::core::mem::transmute(heap), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsCreateListener(channeltype: WS_CHANNEL_TYPE, channelbinding: WS_CHANNEL_BINDING, properties: &[WS_LISTENER_PROPERTY], securitydescription: *const WS_SECURITY_DESCRIPTION, listener: *mut *mut WS_LISTENER, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsCreateListener(channeltype: WS_CHANNEL_TYPE, channelbinding: WS_CHANNEL_BINDING, properties: *const WS_LISTENER_PROPERTY, propertycount: u32, securitydescription: *const WS_SECURITY_DESCRIPTION, listener: *mut *mut WS_LISTENER, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsCreateListener(::core::mem::transmute(channeltype), ::core::mem::transmute(channelbinding), ::core::mem::transmute(::windows::core::as_ptr_or_null(properties)), properties.len() as _, ::core::mem::transmute(securitydescription), ::core::mem::transmute(listener), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsCreateMessage(envelopeversion: WS_ENVELOPE_VERSION, addressingversion: WS_ADDRESSING_VERSION, properties: &[WS_MESSAGE_PROPERTY], message: *mut *mut WS_MESSAGE, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsCreateMessage(envelopeversion: WS_ENVELOPE_VERSION, addressingversion: WS_ADDRESSING_VERSION, properties: *const WS_MESSAGE_PROPERTY, propertycount: u32, message: *mut *mut WS_MESSAGE, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsCreateMessage(::core::mem::transmute(envelopeversion), ::core::mem::transmute(addressingversion), ::core::mem::transmute(::windows::core::as_ptr_or_null(properties)), properties.len() as _, ::core::mem::transmute(message), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsCreateMessageForChannel(channel: *const WS_CHANNEL, properties: &[WS_MESSAGE_PROPERTY], message: *mut *mut WS_MESSAGE, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsCreateMessageForChannel(channel: *const WS_CHANNEL, properties: *const WS_MESSAGE_PROPERTY, propertycount: u32, message: *mut *mut WS_MESSAGE, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsCreateMessageForChannel(::core::mem::transmute(channel), ::core::mem::transmute(::windows::core::as_ptr_or_null(properties)), properties.len() as _, ::core::mem::transmute(message), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsCreateMetadata(properties: &[WS_METADATA_PROPERTY], metadata: *mut *mut WS_METADATA, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsCreateMetadata(properties: *const WS_METADATA_PROPERTY, propertycount: u32, metadata: *mut *mut WS_METADATA, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsCreateMetadata(::core::mem::transmute(::windows::core::as_ptr_or_null(properties)), properties.len() as _, ::core::mem::transmute(metadata), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsCreateReader(properties: &[WS_XML_READER_PROPERTY], reader: *mut *mut WS_XML_READER, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsCreateReader(properties: *const WS_XML_READER_PROPERTY, propertycount: u32, reader: *mut *mut WS_XML_READER, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsCreateReader(::core::mem::transmute(::windows::core::as_ptr_or_null(properties)), properties.len() as _, ::core::mem::transmute(reader), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsCreateServiceEndpointFromTemplate(channeltype: WS_CHANNEL_TYPE, properties: &[WS_SERVICE_ENDPOINT_PROPERTY], addressurl: *const WS_STRING, contract: *const WS_SERVICE_CONTRACT, authorizationcallback: WS_SERVICE_SECURITY_CALLBACK, heap: *const WS_HEAP, templatetype: WS_BINDING_TEMPLATE_TYPE, templatevalue: *const ::core::ffi::c_void, templatesize: u32, templatedescription: *const ::core::ffi::c_void, templatedescriptionsize: u32, serviceendpoint: *mut *mut WS_SERVICE_ENDPOINT, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsCreateServiceEndpointFromTemplate(channeltype: WS_CHANNEL_TYPE, properties: *const WS_SERVICE_ENDPOINT_PROPERTY, propertycount: u32, addressurl: *const WS_STRING, contract: *const WS_SERVICE_CONTRACT, authorizationcallback: *mut ::core::ffi::c_void, heap: *const WS_HEAP, templatetype: WS_BINDING_TEMPLATE_TYPE, templatevalue: *const ::core::ffi::c_void, templatesize: u32, templatedescription: *const ::core::ffi::c_void, templatedescriptionsize: u32, serviceendpoint: *mut *mut WS_SERVICE_ENDPOINT, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsCreateServiceEndpointFromTemplate(
            ::core::mem::transmute(channeltype),
            ::core::mem::transmute(::windows::core::as_ptr_or_null(properties)),
            properties.len() as _,
            ::core::mem::transmute(addressurl),
            ::core::mem::transmute(contract),
            ::core::mem::transmute(authorizationcallback),
            ::core::mem::transmute(heap),
            ::core::mem::transmute(templatetype),
            ::core::mem::transmute(templatevalue),
            ::core::mem::transmute(templatesize),
            ::core::mem::transmute(templatedescription),
            ::core::mem::transmute(templatedescriptionsize),
            ::core::mem::transmute(serviceendpoint),
            ::core::mem::transmute(error),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsCreateServiceHost(endpoints: &[*const WS_SERVICE_ENDPOINT], serviceproperties: &[WS_SERVICE_PROPERTY], servicehost: *mut *mut WS_SERVICE_HOST, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsCreateServiceHost(endpoints: *const *const WS_SERVICE_ENDPOINT, endpointcount: u16, serviceproperties: *const WS_SERVICE_PROPERTY, servicepropertycount: u32, servicehost: *mut *mut WS_SERVICE_HOST, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsCreateServiceHost(::core::mem::transmute(::windows::core::as_ptr_or_null(endpoints)), endpoints.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(serviceproperties)), serviceproperties.len() as _, ::core::mem::transmute(servicehost), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsCreateServiceProxy(channeltype: WS_CHANNEL_TYPE, channelbinding: WS_CHANNEL_BINDING, securitydescription: *const WS_SECURITY_DESCRIPTION, properties: &[WS_PROXY_PROPERTY], channelproperties: &[WS_CHANNEL_PROPERTY], serviceproxy: *mut *mut WS_SERVICE_PROXY, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsCreateServiceProxy(channeltype: WS_CHANNEL_TYPE, channelbinding: WS_CHANNEL_BINDING, securitydescription: *const WS_SECURITY_DESCRIPTION, properties: *const WS_PROXY_PROPERTY, propertycount: u32, channelproperties: *const WS_CHANNEL_PROPERTY, channelpropertycount: u32, serviceproxy: *mut *mut WS_SERVICE_PROXY, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsCreateServiceProxy(::core::mem::transmute(channeltype), ::core::mem::transmute(channelbinding), ::core::mem::transmute(securitydescription), ::core::mem::transmute(::windows::core::as_ptr_or_null(properties)), properties.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(channelproperties)), channelproperties.len() as _, ::core::mem::transmute(serviceproxy), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsCreateServiceProxyFromTemplate(channeltype: WS_CHANNEL_TYPE, properties: &[WS_PROXY_PROPERTY], templatetype: WS_BINDING_TEMPLATE_TYPE, templatevalue: *const ::core::ffi::c_void, templatesize: u32, templatedescription: *const ::core::ffi::c_void, templatedescriptionsize: u32, serviceproxy: *mut *mut WS_SERVICE_PROXY, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsCreateServiceProxyFromTemplate(channeltype: WS_CHANNEL_TYPE, properties: *const WS_PROXY_PROPERTY, propertycount: u32, templatetype: WS_BINDING_TEMPLATE_TYPE, templatevalue: *const ::core::ffi::c_void, templatesize: u32, templatedescription: *const ::core::ffi::c_void, templatedescriptionsize: u32, serviceproxy: *mut *mut WS_SERVICE_PROXY, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsCreateServiceProxyFromTemplate(::core::mem::transmute(channeltype), ::core::mem::transmute(::windows::core::as_ptr_or_null(properties)), properties.len() as _, ::core::mem::transmute(templatetype), ::core::mem::transmute(templatevalue), ::core::mem::transmute(templatesize), ::core::mem::transmute(templatedescription), ::core::mem::transmute(templatedescriptionsize), ::core::mem::transmute(serviceproxy), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsCreateWriter(properties: &[WS_XML_WRITER_PROPERTY], writer: *mut *mut WS_XML_WRITER, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsCreateWriter(properties: *const WS_XML_WRITER_PROPERTY, propertycount: u32, writer: *mut *mut WS_XML_WRITER, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsCreateWriter(::core::mem::transmute(::windows::core::as_ptr_or_null(properties)), properties.len() as _, ::core::mem::transmute(writer), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsCreateXmlBuffer(heap: *const WS_HEAP, properties: &[WS_XML_BUFFER_PROPERTY], buffer: *mut *mut WS_XML_BUFFER, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsCreateXmlBuffer(heap: *const WS_HEAP, properties: *const WS_XML_BUFFER_PROPERTY, propertycount: u32, buffer: *mut *mut WS_XML_BUFFER, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsCreateXmlBuffer(::core::mem::transmute(heap), ::core::mem::transmute(::windows::core::as_ptr_or_null(properties)), properties.len() as _, ::core::mem::transmute(buffer), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsCreateXmlSecurityToken(tokenxml: *const WS_XML_BUFFER, tokenkey: *const WS_SECURITY_KEY_HANDLE, properties: &[WS_XML_SECURITY_TOKEN_PROPERTY], token: *mut *mut WS_SECURITY_TOKEN, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsCreateXmlSecurityToken(tokenxml: *const WS_XML_BUFFER, tokenkey: *const WS_SECURITY_KEY_HANDLE, properties: *const WS_XML_SECURITY_TOKEN_PROPERTY, propertycount: u32, token: *mut *mut WS_SECURITY_TOKEN, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsCreateXmlSecurityToken(::core::mem::transmute(tokenxml), ::core::mem::transmute(tokenkey), ::core::mem::transmute(::windows::core::as_ptr_or_null(properties)), properties.len() as _, ::core::mem::transmute(token), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsDateTimeToFileTime(datetime: *const WS_DATETIME, filetime: *mut super::super::Foundation::FILETIME, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsDateTimeToFileTime(datetime: *const WS_DATETIME, filetime: *mut super::super::Foundation::FILETIME, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsDateTimeToFileTime(::core::mem::transmute(datetime), ::core::mem::transmute(filetime), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsDecodeUrl(url: *const WS_STRING, flags: u32, heap: *const WS_HEAP, outurl: *mut *mut WS_URL, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsDecodeUrl(url: *const WS_STRING, flags: u32, heap: *const WS_HEAP, outurl: *mut *mut WS_URL, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsDecodeUrl(::core::mem::transmute(url), ::core::mem::transmute(flags), ::core::mem::transmute(heap), ::core::mem::transmute(outurl), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsEncodeUrl(url: *const WS_URL, flags: u32, heap: *const WS_HEAP, outurl: *mut WS_STRING, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsEncodeUrl(url: *const WS_URL, flags: u32, heap: *const WS_HEAP, outurl: *mut WS_STRING, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsEncodeUrl(::core::mem::transmute(url), ::core::mem::transmute(flags), ::core::mem::transmute(heap), ::core::mem::transmute(outurl), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsEndReaderCanonicalization(reader: *const WS_XML_READER, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsEndReaderCanonicalization(reader: *const WS_XML_READER, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsEndReaderCanonicalization(::core::mem::transmute(reader), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsEndWriterCanonicalization(writer: *const WS_XML_WRITER, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsEndWriterCanonicalization(writer: *const WS_XML_WRITER, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsEndWriterCanonicalization(::core::mem::transmute(writer), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsFileTimeToDateTime(filetime: *const super::super::Foundation::FILETIME, datetime: *mut WS_DATETIME, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsFileTimeToDateTime(filetime: *const super::super::Foundation::FILETIME, datetime: *mut WS_DATETIME, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsFileTimeToDateTime(::core::mem::transmute(filetime), ::core::mem::transmute(datetime), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsFillBody(message: *const WS_MESSAGE, minsize: u32, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsFillBody(message: *const WS_MESSAGE, minsize: u32, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsFillBody(::core::mem::transmute(message), ::core::mem::transmute(minsize), ::core::mem::transmute(asynccontext), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsFillReader(reader: *const WS_XML_READER, minsize: u32, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsFillReader(reader: *const WS_XML_READER, minsize: u32, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsFillReader(::core::mem::transmute(reader), ::core::mem::transmute(minsize), ::core::mem::transmute(asynccontext), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsFindAttribute<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(reader: *const WS_XML_READER, localname: *const WS_XML_STRING, ns: *const WS_XML_STRING, required: Param3, attributeindex: *mut u32, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsFindAttribute(reader: *const WS_XML_READER, localname: *const WS_XML_STRING, ns: *const WS_XML_STRING, required: super::super::Foundation::BOOL, attributeindex: *mut u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsFindAttribute(::core::mem::transmute(reader), ::core::mem::transmute(localname), ::core::mem::transmute(ns), required.into_param().abi(), ::core::mem::transmute(attributeindex), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsFlushBody(message: *const WS_MESSAGE, minsize: u32, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsFlushBody(message: *const WS_MESSAGE, minsize: u32, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsFlushBody(::core::mem::transmute(message), ::core::mem::transmute(minsize), ::core::mem::transmute(asynccontext), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsFlushWriter(writer: *const WS_XML_WRITER, minsize: u32, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsFlushWriter(writer: *const WS_XML_WRITER, minsize: u32, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsFlushWriter(::core::mem::transmute(writer), ::core::mem::transmute(minsize), ::core::mem::transmute(asynccontext), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsFreeChannel(channel: *const WS_CHANNEL) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsFreeChannel(channel: *const WS_CHANNEL);
        }
        WsFreeChannel(::core::mem::transmute(channel))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsFreeError(error: *const WS_ERROR) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsFreeError(error: *const WS_ERROR);
        }
        WsFreeError(::core::mem::transmute(error))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsFreeHeap(heap: *const WS_HEAP) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsFreeHeap(heap: *const WS_HEAP);
        }
        WsFreeHeap(::core::mem::transmute(heap))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsFreeListener(listener: *const WS_LISTENER) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsFreeListener(listener: *const WS_LISTENER);
        }
        WsFreeListener(::core::mem::transmute(listener))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsFreeMessage(message: *const WS_MESSAGE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsFreeMessage(message: *const WS_MESSAGE);
        }
        WsFreeMessage(::core::mem::transmute(message))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsFreeMetadata(metadata: *const WS_METADATA) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsFreeMetadata(metadata: *const WS_METADATA);
        }
        WsFreeMetadata(::core::mem::transmute(metadata))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsFreeReader(reader: *const WS_XML_READER) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsFreeReader(reader: *const WS_XML_READER);
        }
        WsFreeReader(::core::mem::transmute(reader))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsFreeSecurityToken(token: *const WS_SECURITY_TOKEN) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsFreeSecurityToken(token: *const WS_SECURITY_TOKEN);
        }
        WsFreeSecurityToken(::core::mem::transmute(token))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsFreeServiceHost(servicehost: *const WS_SERVICE_HOST) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsFreeServiceHost(servicehost: *const WS_SERVICE_HOST);
        }
        WsFreeServiceHost(::core::mem::transmute(servicehost))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsFreeServiceProxy(serviceproxy: *const WS_SERVICE_PROXY) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsFreeServiceProxy(serviceproxy: *const WS_SERVICE_PROXY);
        }
        WsFreeServiceProxy(::core::mem::transmute(serviceproxy))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsFreeWriter(writer: *const WS_XML_WRITER) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsFreeWriter(writer: *const WS_XML_WRITER);
        }
        WsFreeWriter(::core::mem::transmute(writer))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsGetChannelProperty(channel: *const WS_CHANNEL, id: WS_CHANNEL_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsGetChannelProperty(channel: *const WS_CHANNEL, id: WS_CHANNEL_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsGetChannelProperty(::core::mem::transmute(channel), ::core::mem::transmute(id), ::core::mem::transmute(value), ::core::mem::transmute(valuesize), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsGetCustomHeader(message: *const WS_MESSAGE, customheaderdescription: *const WS_ELEMENT_DESCRIPTION, repeatingoption: WS_REPEATING_HEADER_OPTION, headerindex: u32, readoption: WS_READ_OPTION, heap: *const WS_HEAP, value: *mut ::core::ffi::c_void, valuesize: u32, headerattributes: *mut u32, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsGetCustomHeader(message: *const WS_MESSAGE, customheaderdescription: *const WS_ELEMENT_DESCRIPTION, repeatingoption: WS_REPEATING_HEADER_OPTION, headerindex: u32, readoption: WS_READ_OPTION, heap: *const WS_HEAP, value: *mut ::core::ffi::c_void, valuesize: u32, headerattributes: *mut u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsGetCustomHeader(::core::mem::transmute(message), ::core::mem::transmute(customheaderdescription), ::core::mem::transmute(repeatingoption), ::core::mem::transmute(headerindex), ::core::mem::transmute(readoption), ::core::mem::transmute(heap), ::core::mem::transmute(value), ::core::mem::transmute(valuesize), ::core::mem::transmute(headerattributes), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsGetDictionary(encoding: WS_ENCODING, dictionary: *mut *mut WS_XML_DICTIONARY, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsGetDictionary(encoding: WS_ENCODING, dictionary: *mut *mut WS_XML_DICTIONARY, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsGetDictionary(::core::mem::transmute(encoding), ::core::mem::transmute(dictionary), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsGetErrorProperty(error: *const WS_ERROR, id: WS_ERROR_PROPERTY_ID, buffer: *mut ::core::ffi::c_void, buffersize: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsGetErrorProperty(error: *const WS_ERROR, id: WS_ERROR_PROPERTY_ID, buffer: *mut ::core::ffi::c_void, buffersize: u32) -> ::windows::core::HRESULT;
        }
        WsGetErrorProperty(::core::mem::transmute(error), ::core::mem::transmute(id), ::core::mem::transmute(buffer), ::core::mem::transmute(buffersize)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsGetErrorString(error: *const WS_ERROR, index: u32) -> ::windows::core::Result<WS_STRING> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsGetErrorString(error: *const WS_ERROR, index: u32, string: *mut WS_STRING) -> ::windows::core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<WS_STRING>::zeroed();
        WsGetErrorString(::core::mem::transmute(error), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WS_STRING>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsGetFaultErrorDetail(error: *const WS_ERROR, faultdetaildescription: *const WS_FAULT_DETAIL_DESCRIPTION, readoption: WS_READ_OPTION, heap: *const WS_HEAP, value: *mut ::core::ffi::c_void, valuesize: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsGetFaultErrorDetail(error: *const WS_ERROR, faultdetaildescription: *const WS_FAULT_DETAIL_DESCRIPTION, readoption: WS_READ_OPTION, heap: *const WS_HEAP, value: *mut ::core::ffi::c_void, valuesize: u32) -> ::windows::core::HRESULT;
        }
        WsGetFaultErrorDetail(::core::mem::transmute(error), ::core::mem::transmute(faultdetaildescription), ::core::mem::transmute(readoption), ::core::mem::transmute(heap), ::core::mem::transmute(value), ::core::mem::transmute(valuesize)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsGetFaultErrorProperty(error: *const WS_ERROR, id: WS_FAULT_ERROR_PROPERTY_ID, buffer: *mut ::core::ffi::c_void, buffersize: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsGetFaultErrorProperty(error: *const WS_ERROR, id: WS_FAULT_ERROR_PROPERTY_ID, buffer: *mut ::core::ffi::c_void, buffersize: u32) -> ::windows::core::HRESULT;
        }
        WsGetFaultErrorProperty(::core::mem::transmute(error), ::core::mem::transmute(id), ::core::mem::transmute(buffer), ::core::mem::transmute(buffersize)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsGetHeader(message: *const WS_MESSAGE, headertype: WS_HEADER_TYPE, valuetype: WS_TYPE, readoption: WS_READ_OPTION, heap: *const WS_HEAP, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsGetHeader(message: *const WS_MESSAGE, headertype: WS_HEADER_TYPE, valuetype: WS_TYPE, readoption: WS_READ_OPTION, heap: *const WS_HEAP, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsGetHeader(::core::mem::transmute(message), ::core::mem::transmute(headertype), ::core::mem::transmute(valuetype), ::core::mem::transmute(readoption), ::core::mem::transmute(heap), ::core::mem::transmute(value), ::core::mem::transmute(valuesize), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsGetHeaderAttributes(message: *const WS_MESSAGE, reader: *const WS_XML_READER, headerattributes: *mut u32, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsGetHeaderAttributes(message: *const WS_MESSAGE, reader: *const WS_XML_READER, headerattributes: *mut u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsGetHeaderAttributes(::core::mem::transmute(message), ::core::mem::transmute(reader), ::core::mem::transmute(headerattributes), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsGetHeapProperty(heap: *const WS_HEAP, id: WS_HEAP_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsGetHeapProperty(heap: *const WS_HEAP, id: WS_HEAP_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsGetHeapProperty(::core::mem::transmute(heap), ::core::mem::transmute(id), ::core::mem::transmute(value), ::core::mem::transmute(valuesize), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsGetListenerProperty(listener: *const WS_LISTENER, id: WS_LISTENER_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsGetListenerProperty(listener: *const WS_LISTENER, id: WS_LISTENER_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsGetListenerProperty(::core::mem::transmute(listener), ::core::mem::transmute(id), ::core::mem::transmute(value), ::core::mem::transmute(valuesize), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsGetMappedHeader(message: *const WS_MESSAGE, headername: *const WS_XML_STRING, repeatingoption: WS_REPEATING_HEADER_OPTION, headerindex: u32, valuetype: WS_TYPE, readoption: WS_READ_OPTION, heap: *const WS_HEAP, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsGetMappedHeader(message: *const WS_MESSAGE, headername: *const WS_XML_STRING, repeatingoption: WS_REPEATING_HEADER_OPTION, headerindex: u32, valuetype: WS_TYPE, readoption: WS_READ_OPTION, heap: *const WS_HEAP, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsGetMappedHeader(::core::mem::transmute(message), ::core::mem::transmute(headername), ::core::mem::transmute(repeatingoption), ::core::mem::transmute(headerindex), ::core::mem::transmute(valuetype), ::core::mem::transmute(readoption), ::core::mem::transmute(heap), ::core::mem::transmute(value), ::core::mem::transmute(valuesize), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsGetMessageProperty(message: *const WS_MESSAGE, id: WS_MESSAGE_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsGetMessageProperty(message: *const WS_MESSAGE, id: WS_MESSAGE_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsGetMessageProperty(::core::mem::transmute(message), ::core::mem::transmute(id), ::core::mem::transmute(value), ::core::mem::transmute(valuesize), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsGetMetadataEndpoints(metadata: *const WS_METADATA, endpoints: *mut WS_METADATA_ENDPOINTS, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsGetMetadataEndpoints(metadata: *const WS_METADATA, endpoints: *mut WS_METADATA_ENDPOINTS, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsGetMetadataEndpoints(::core::mem::transmute(metadata), ::core::mem::transmute(endpoints), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsGetMetadataProperty(metadata: *const WS_METADATA, id: WS_METADATA_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsGetMetadataProperty(metadata: *const WS_METADATA, id: WS_METADATA_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsGetMetadataProperty(::core::mem::transmute(metadata), ::core::mem::transmute(id), ::core::mem::transmute(value), ::core::mem::transmute(valuesize), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsGetMissingMetadataDocumentAddress(metadata: *const WS_METADATA, address: *mut *mut WS_ENDPOINT_ADDRESS, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsGetMissingMetadataDocumentAddress(metadata: *const WS_METADATA, address: *mut *mut WS_ENDPOINT_ADDRESS, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsGetMissingMetadataDocumentAddress(::core::mem::transmute(metadata), ::core::mem::transmute(address), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsGetNamespaceFromPrefix<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(reader: *const WS_XML_READER, prefix: *const WS_XML_STRING, required: Param2, ns: *mut *mut WS_XML_STRING, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsGetNamespaceFromPrefix(reader: *const WS_XML_READER, prefix: *const WS_XML_STRING, required: super::super::Foundation::BOOL, ns: *mut *mut WS_XML_STRING, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsGetNamespaceFromPrefix(::core::mem::transmute(reader), ::core::mem::transmute(prefix), required.into_param().abi(), ::core::mem::transmute(ns), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsGetOperationContextProperty(context: *const WS_OPERATION_CONTEXT, id: WS_OPERATION_CONTEXT_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsGetOperationContextProperty(context: *const WS_OPERATION_CONTEXT, id: WS_OPERATION_CONTEXT_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsGetOperationContextProperty(::core::mem::transmute(context), ::core::mem::transmute(id), ::core::mem::transmute(value), ::core::mem::transmute(valuesize), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsGetPolicyAlternativeCount(policy: *const WS_POLICY, count: *mut u32, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsGetPolicyAlternativeCount(policy: *const WS_POLICY, count: *mut u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsGetPolicyAlternativeCount(::core::mem::transmute(policy), ::core::mem::transmute(count), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsGetPolicyProperty(policy: *const WS_POLICY, id: WS_POLICY_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsGetPolicyProperty(policy: *const WS_POLICY, id: WS_POLICY_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsGetPolicyProperty(::core::mem::transmute(policy), ::core::mem::transmute(id), ::core::mem::transmute(value), ::core::mem::transmute(valuesize), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsGetPrefixFromNamespace<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(writer: *const WS_XML_WRITER, ns: *const WS_XML_STRING, required: Param2, prefix: *mut *mut WS_XML_STRING, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsGetPrefixFromNamespace(writer: *const WS_XML_WRITER, ns: *const WS_XML_STRING, required: super::super::Foundation::BOOL, prefix: *mut *mut WS_XML_STRING, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsGetPrefixFromNamespace(::core::mem::transmute(writer), ::core::mem::transmute(ns), required.into_param().abi(), ::core::mem::transmute(prefix), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsGetReaderNode(xmlreader: *const WS_XML_READER, node: *mut *mut WS_XML_NODE, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsGetReaderNode(xmlreader: *const WS_XML_READER, node: *mut *mut WS_XML_NODE, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsGetReaderNode(::core::mem::transmute(xmlreader), ::core::mem::transmute(node), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsGetReaderPosition(reader: *const WS_XML_READER, nodeposition: *mut WS_XML_NODE_POSITION, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsGetReaderPosition(reader: *const WS_XML_READER, nodeposition: *mut WS_XML_NODE_POSITION, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsGetReaderPosition(::core::mem::transmute(reader), ::core::mem::transmute(nodeposition), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsGetReaderProperty(reader: *const WS_XML_READER, id: WS_XML_READER_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsGetReaderProperty(reader: *const WS_XML_READER, id: WS_XML_READER_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsGetReaderProperty(::core::mem::transmute(reader), ::core::mem::transmute(id), ::core::mem::transmute(value), ::core::mem::transmute(valuesize), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsGetSecurityContextProperty(securitycontext: *const WS_SECURITY_CONTEXT, id: WS_SECURITY_CONTEXT_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsGetSecurityContextProperty(securitycontext: *const WS_SECURITY_CONTEXT, id: WS_SECURITY_CONTEXT_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsGetSecurityContextProperty(::core::mem::transmute(securitycontext), ::core::mem::transmute(id), ::core::mem::transmute(value), ::core::mem::transmute(valuesize), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsGetSecurityTokenProperty(securitytoken: *const WS_SECURITY_TOKEN, id: WS_SECURITY_TOKEN_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, heap: *const WS_HEAP, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsGetSecurityTokenProperty(securitytoken: *const WS_SECURITY_TOKEN, id: WS_SECURITY_TOKEN_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, heap: *const WS_HEAP, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsGetSecurityTokenProperty(::core::mem::transmute(securitytoken), ::core::mem::transmute(id), ::core::mem::transmute(value), ::core::mem::transmute(valuesize), ::core::mem::transmute(heap), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsGetServiceHostProperty(servicehost: *const WS_SERVICE_HOST, id: WS_SERVICE_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsGetServiceHostProperty(servicehost: *const WS_SERVICE_HOST, id: WS_SERVICE_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsGetServiceHostProperty(::core::mem::transmute(servicehost), ::core::mem::transmute(id), ::core::mem::transmute(value), ::core::mem::transmute(valuesize), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsGetServiceProxyProperty(serviceproxy: *const WS_SERVICE_PROXY, id: WS_PROXY_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsGetServiceProxyProperty(serviceproxy: *const WS_SERVICE_PROXY, id: WS_PROXY_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsGetServiceProxyProperty(::core::mem::transmute(serviceproxy), ::core::mem::transmute(id), ::core::mem::transmute(value), ::core::mem::transmute(valuesize), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsGetWriterPosition(writer: *const WS_XML_WRITER, nodeposition: *mut WS_XML_NODE_POSITION, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsGetWriterPosition(writer: *const WS_XML_WRITER, nodeposition: *mut WS_XML_NODE_POSITION, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsGetWriterPosition(::core::mem::transmute(writer), ::core::mem::transmute(nodeposition), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsGetWriterProperty(writer: *const WS_XML_WRITER, id: WS_XML_WRITER_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsGetWriterProperty(writer: *const WS_XML_WRITER, id: WS_XML_WRITER_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsGetWriterProperty(::core::mem::transmute(writer), ::core::mem::transmute(id), ::core::mem::transmute(value), ::core::mem::transmute(valuesize), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsGetXmlAttribute(reader: *const WS_XML_READER, localname: *const WS_XML_STRING, heap: *const WS_HEAP, valuechars: *mut *mut u16, valuecharcount: *mut u32, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsGetXmlAttribute(reader: *const WS_XML_READER, localname: *const WS_XML_STRING, heap: *const WS_HEAP, valuechars: *mut *mut u16, valuecharcount: *mut u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsGetXmlAttribute(::core::mem::transmute(reader), ::core::mem::transmute(localname), ::core::mem::transmute(heap), ::core::mem::transmute(valuechars), ::core::mem::transmute(valuecharcount), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsInitializeMessage(message: *const WS_MESSAGE, initialization: WS_MESSAGE_INITIALIZATION, sourcemessage: *const WS_MESSAGE, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsInitializeMessage(message: *const WS_MESSAGE, initialization: WS_MESSAGE_INITIALIZATION, sourcemessage: *const WS_MESSAGE, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsInitializeMessage(::core::mem::transmute(message), ::core::mem::transmute(initialization), ::core::mem::transmute(sourcemessage), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsMarkHeaderAsUnderstood(message: *const WS_MESSAGE, headerposition: *const WS_XML_NODE_POSITION, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsMarkHeaderAsUnderstood(message: *const WS_MESSAGE, headerposition: *const WS_XML_NODE_POSITION, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsMarkHeaderAsUnderstood(::core::mem::transmute(message), ::core::mem::transmute(headerposition), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsMatchPolicyAlternative<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(policy: *const WS_POLICY, alternativeindex: u32, policyconstraints: *const WS_POLICY_CONSTRAINTS, matchrequired: Param3, heap: *const WS_HEAP, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsMatchPolicyAlternative(policy: *const WS_POLICY, alternativeindex: u32, policyconstraints: *const WS_POLICY_CONSTRAINTS, matchrequired: super::super::Foundation::BOOL, heap: *const WS_HEAP, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsMatchPolicyAlternative(::core::mem::transmute(policy), ::core::mem::transmute(alternativeindex), ::core::mem::transmute(policyconstraints), matchrequired.into_param().abi(), ::core::mem::transmute(heap), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsMoveReader(reader: *const WS_XML_READER, moveto: WS_MOVE_TO, found: *mut super::super::Foundation::BOOL, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsMoveReader(reader: *const WS_XML_READER, moveto: WS_MOVE_TO, found: *mut super::super::Foundation::BOOL, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsMoveReader(::core::mem::transmute(reader), ::core::mem::transmute(moveto), ::core::mem::transmute(found), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsMoveWriter(writer: *const WS_XML_WRITER, moveto: WS_MOVE_TO, found: *mut super::super::Foundation::BOOL, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsMoveWriter(writer: *const WS_XML_WRITER, moveto: WS_MOVE_TO, found: *mut super::super::Foundation::BOOL, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsMoveWriter(::core::mem::transmute(writer), ::core::mem::transmute(moveto), ::core::mem::transmute(found), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsOpenChannel(channel: *const WS_CHANNEL, endpointaddress: *const WS_ENDPOINT_ADDRESS, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsOpenChannel(channel: *const WS_CHANNEL, endpointaddress: *const WS_ENDPOINT_ADDRESS, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsOpenChannel(::core::mem::transmute(channel), ::core::mem::transmute(endpointaddress), ::core::mem::transmute(asynccontext), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsOpenListener(listener: *const WS_LISTENER, url: *const WS_STRING, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsOpenListener(listener: *const WS_LISTENER, url: *const WS_STRING, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsOpenListener(::core::mem::transmute(listener), ::core::mem::transmute(url), ::core::mem::transmute(asynccontext), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsOpenServiceHost(servicehost: *const WS_SERVICE_HOST, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsOpenServiceHost(servicehost: *const WS_SERVICE_HOST, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsOpenServiceHost(::core::mem::transmute(servicehost), ::core::mem::transmute(asynccontext), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsOpenServiceProxy(serviceproxy: *const WS_SERVICE_PROXY, address: *const WS_ENDPOINT_ADDRESS, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsOpenServiceProxy(serviceproxy: *const WS_SERVICE_PROXY, address: *const WS_ENDPOINT_ADDRESS, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsOpenServiceProxy(::core::mem::transmute(serviceproxy), ::core::mem::transmute(address), ::core::mem::transmute(asynccontext), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsPullBytes(writer: *const WS_XML_WRITER, callback: WS_PULL_BYTES_CALLBACK, callbackstate: *const ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsPullBytes(writer: *const WS_XML_WRITER, callback: *mut ::core::ffi::c_void, callbackstate: *const ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsPullBytes(::core::mem::transmute(writer), ::core::mem::transmute(callback), ::core::mem::transmute(callbackstate), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsPushBytes(writer: *const WS_XML_WRITER, callback: WS_PUSH_BYTES_CALLBACK, callbackstate: *const ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsPushBytes(writer: *const WS_XML_WRITER, callback: *mut ::core::ffi::c_void, callbackstate: *const ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsPushBytes(::core::mem::transmute(writer), ::core::mem::transmute(callback), ::core::mem::transmute(callbackstate), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsReadArray(reader: *const WS_XML_READER, localname: *const WS_XML_STRING, ns: *const WS_XML_STRING, valuetype: WS_VALUE_TYPE, array: *mut ::core::ffi::c_void, arraysize: u32, itemoffset: u32, itemcount: u32, actualitemcount: *mut u32, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsReadArray(reader: *const WS_XML_READER, localname: *const WS_XML_STRING, ns: *const WS_XML_STRING, valuetype: WS_VALUE_TYPE, array: *mut ::core::ffi::c_void, arraysize: u32, itemoffset: u32, itemcount: u32, actualitemcount: *mut u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsReadArray(::core::mem::transmute(reader), ::core::mem::transmute(localname), ::core::mem::transmute(ns), ::core::mem::transmute(valuetype), ::core::mem::transmute(array), ::core::mem::transmute(arraysize), ::core::mem::transmute(itemoffset), ::core::mem::transmute(itemcount), ::core::mem::transmute(actualitemcount), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsReadAttribute(reader: *const WS_XML_READER, attributedescription: *const WS_ATTRIBUTE_DESCRIPTION, readoption: WS_READ_OPTION, heap: *const WS_HEAP, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsReadAttribute(reader: *const WS_XML_READER, attributedescription: *const WS_ATTRIBUTE_DESCRIPTION, readoption: WS_READ_OPTION, heap: *const WS_HEAP, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsReadAttribute(::core::mem::transmute(reader), ::core::mem::transmute(attributedescription), ::core::mem::transmute(readoption), ::core::mem::transmute(heap), ::core::mem::transmute(value), ::core::mem::transmute(valuesize), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsReadBody(message: *const WS_MESSAGE, bodydescription: *const WS_ELEMENT_DESCRIPTION, readoption: WS_READ_OPTION, heap: *const WS_HEAP, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsReadBody(message: *const WS_MESSAGE, bodydescription: *const WS_ELEMENT_DESCRIPTION, readoption: WS_READ_OPTION, heap: *const WS_HEAP, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsReadBody(::core::mem::transmute(message), ::core::mem::transmute(bodydescription), ::core::mem::transmute(readoption), ::core::mem::transmute(heap), ::core::mem::transmute(value), ::core::mem::transmute(valuesize), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsReadBytes(reader: *const WS_XML_READER, bytes: *mut ::core::ffi::c_void, maxbytecount: u32, actualbytecount: *mut u32, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsReadBytes(reader: *const WS_XML_READER, bytes: *mut ::core::ffi::c_void, maxbytecount: u32, actualbytecount: *mut u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsReadBytes(::core::mem::transmute(reader), ::core::mem::transmute(bytes), ::core::mem::transmute(maxbytecount), ::core::mem::transmute(actualbytecount), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsReadChars(reader: *const WS_XML_READER, chars: &mut [u16], actualcharcount: *mut u32, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsReadChars(reader: *const WS_XML_READER, chars: ::windows::core::PWSTR, maxcharcount: u32, actualcharcount: *mut u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsReadChars(::core::mem::transmute(reader), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(chars)), chars.len() as _, ::core::mem::transmute(actualcharcount), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsReadCharsUtf8(reader: *const WS_XML_READER, bytes: &mut [u8], actualbytecount: *mut u32, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsReadCharsUtf8(reader: *const WS_XML_READER, bytes: *mut u8, maxbytecount: u32, actualbytecount: *mut u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsReadCharsUtf8(::core::mem::transmute(reader), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(bytes)), bytes.len() as _, ::core::mem::transmute(actualbytecount), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsReadElement(reader: *const WS_XML_READER, elementdescription: *const WS_ELEMENT_DESCRIPTION, readoption: WS_READ_OPTION, heap: *const WS_HEAP, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsReadElement(reader: *const WS_XML_READER, elementdescription: *const WS_ELEMENT_DESCRIPTION, readoption: WS_READ_OPTION, heap: *const WS_HEAP, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsReadElement(::core::mem::transmute(reader), ::core::mem::transmute(elementdescription), ::core::mem::transmute(readoption), ::core::mem::transmute(heap), ::core::mem::transmute(value), ::core::mem::transmute(valuesize), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsReadEndAttribute(reader: *const WS_XML_READER, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsReadEndAttribute(reader: *const WS_XML_READER, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsReadEndAttribute(::core::mem::transmute(reader), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsReadEndElement(reader: *const WS_XML_READER, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsReadEndElement(reader: *const WS_XML_READER, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsReadEndElement(::core::mem::transmute(reader), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsReadEndpointAddressExtension(reader: *const WS_XML_READER, endpointaddress: *const WS_ENDPOINT_ADDRESS, extensiontype: WS_ENDPOINT_ADDRESS_EXTENSION_TYPE, readoption: WS_READ_OPTION, heap: *const WS_HEAP, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsReadEndpointAddressExtension(reader: *const WS_XML_READER, endpointaddress: *const WS_ENDPOINT_ADDRESS, extensiontype: WS_ENDPOINT_ADDRESS_EXTENSION_TYPE, readoption: WS_READ_OPTION, heap: *const WS_HEAP, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsReadEndpointAddressExtension(::core::mem::transmute(reader), ::core::mem::transmute(endpointaddress), ::core::mem::transmute(extensiontype), ::core::mem::transmute(readoption), ::core::mem::transmute(heap), ::core::mem::transmute(value), ::core::mem::transmute(valuesize), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsReadEnvelopeEnd(message: *const WS_MESSAGE, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsReadEnvelopeEnd(message: *const WS_MESSAGE, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsReadEnvelopeEnd(::core::mem::transmute(message), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsReadEnvelopeStart(message: *const WS_MESSAGE, reader: *const WS_XML_READER, donecallback: WS_MESSAGE_DONE_CALLBACK, donecallbackstate: *const ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsReadEnvelopeStart(message: *const WS_MESSAGE, reader: *const WS_XML_READER, donecallback: *mut ::core::ffi::c_void, donecallbackstate: *const ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsReadEnvelopeStart(::core::mem::transmute(message), ::core::mem::transmute(reader), ::core::mem::transmute(donecallback), ::core::mem::transmute(donecallbackstate), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsReadMessageEnd(channel: *const WS_CHANNEL, message: *const WS_MESSAGE, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsReadMessageEnd(channel: *const WS_CHANNEL, message: *const WS_MESSAGE, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsReadMessageEnd(::core::mem::transmute(channel), ::core::mem::transmute(message), ::core::mem::transmute(asynccontext), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsReadMessageStart(channel: *const WS_CHANNEL, message: *const WS_MESSAGE, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsReadMessageStart(channel: *const WS_CHANNEL, message: *const WS_MESSAGE, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsReadMessageStart(::core::mem::transmute(channel), ::core::mem::transmute(message), ::core::mem::transmute(asynccontext), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsReadMetadata(metadata: *const WS_METADATA, reader: *const WS_XML_READER, url: *const WS_STRING, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsReadMetadata(metadata: *const WS_METADATA, reader: *const WS_XML_READER, url: *const WS_STRING, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsReadMetadata(::core::mem::transmute(metadata), ::core::mem::transmute(reader), ::core::mem::transmute(url), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsReadNode(reader: *const WS_XML_READER, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsReadNode(reader: *const WS_XML_READER, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsReadNode(::core::mem::transmute(reader), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsReadQualifiedName(reader: *const WS_XML_READER, heap: *const WS_HEAP, prefix: *mut WS_XML_STRING, localname: *mut WS_XML_STRING, ns: *mut WS_XML_STRING, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsReadQualifiedName(reader: *const WS_XML_READER, heap: *const WS_HEAP, prefix: *mut WS_XML_STRING, localname: *mut WS_XML_STRING, ns: *mut WS_XML_STRING, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsReadQualifiedName(::core::mem::transmute(reader), ::core::mem::transmute(heap), ::core::mem::transmute(prefix), ::core::mem::transmute(localname), ::core::mem::transmute(ns), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsReadStartAttribute(reader: *const WS_XML_READER, attributeindex: u32, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsReadStartAttribute(reader: *const WS_XML_READER, attributeindex: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsReadStartAttribute(::core::mem::transmute(reader), ::core::mem::transmute(attributeindex), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsReadStartElement(reader: *const WS_XML_READER, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsReadStartElement(reader: *const WS_XML_READER, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsReadStartElement(::core::mem::transmute(reader), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsReadToStartElement(reader: *const WS_XML_READER, localname: *const WS_XML_STRING, ns: *const WS_XML_STRING, found: *mut super::super::Foundation::BOOL, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsReadToStartElement(reader: *const WS_XML_READER, localname: *const WS_XML_STRING, ns: *const WS_XML_STRING, found: *mut super::super::Foundation::BOOL, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsReadToStartElement(::core::mem::transmute(reader), ::core::mem::transmute(localname), ::core::mem::transmute(ns), ::core::mem::transmute(found), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsReadType(reader: *const WS_XML_READER, typemapping: WS_TYPE_MAPPING, r#type: WS_TYPE, typedescription: *const ::core::ffi::c_void, readoption: WS_READ_OPTION, heap: *const WS_HEAP, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsReadType(reader: *const WS_XML_READER, typemapping: WS_TYPE_MAPPING, r#type: WS_TYPE, typedescription: *const ::core::ffi::c_void, readoption: WS_READ_OPTION, heap: *const WS_HEAP, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsReadType(::core::mem::transmute(reader), ::core::mem::transmute(typemapping), ::core::mem::transmute(r#type), ::core::mem::transmute(typedescription), ::core::mem::transmute(readoption), ::core::mem::transmute(heap), ::core::mem::transmute(value), ::core::mem::transmute(valuesize), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsReadValue(reader: *const WS_XML_READER, valuetype: WS_VALUE_TYPE, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsReadValue(reader: *const WS_XML_READER, valuetype: WS_VALUE_TYPE, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsReadValue(::core::mem::transmute(reader), ::core::mem::transmute(valuetype), ::core::mem::transmute(value), ::core::mem::transmute(valuesize), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsReadXmlBuffer(reader: *const WS_XML_READER, heap: *const WS_HEAP, xmlbuffer: *mut *mut WS_XML_BUFFER, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsReadXmlBuffer(reader: *const WS_XML_READER, heap: *const WS_HEAP, xmlbuffer: *mut *mut WS_XML_BUFFER, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsReadXmlBuffer(::core::mem::transmute(reader), ::core::mem::transmute(heap), ::core::mem::transmute(xmlbuffer), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsReadXmlBufferFromBytes(reader: *const WS_XML_READER, encoding: *const WS_XML_READER_ENCODING, properties: &[WS_XML_READER_PROPERTY], bytes: *const ::core::ffi::c_void, bytecount: u32, heap: *const WS_HEAP, xmlbuffer: *mut *mut WS_XML_BUFFER, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsReadXmlBufferFromBytes(reader: *const WS_XML_READER, encoding: *const WS_XML_READER_ENCODING, properties: *const WS_XML_READER_PROPERTY, propertycount: u32, bytes: *const ::core::ffi::c_void, bytecount: u32, heap: *const WS_HEAP, xmlbuffer: *mut *mut WS_XML_BUFFER, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsReadXmlBufferFromBytes(::core::mem::transmute(reader), ::core::mem::transmute(encoding), ::core::mem::transmute(::windows::core::as_ptr_or_null(properties)), properties.len() as _, ::core::mem::transmute(bytes), ::core::mem::transmute(bytecount), ::core::mem::transmute(heap), ::core::mem::transmute(xmlbuffer), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsReceiveMessage(channel: *const WS_CHANNEL, message: *const WS_MESSAGE, messagedescriptions: &[*const WS_MESSAGE_DESCRIPTION], receiveoption: WS_RECEIVE_OPTION, readbodyoption: WS_READ_OPTION, heap: *const WS_HEAP, value: *mut ::core::ffi::c_void, valuesize: u32, index: *mut u32, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsReceiveMessage(channel: *const WS_CHANNEL, message: *const WS_MESSAGE, messagedescriptions: *const *const WS_MESSAGE_DESCRIPTION, messagedescriptioncount: u32, receiveoption: WS_RECEIVE_OPTION, readbodyoption: WS_READ_OPTION, heap: *const WS_HEAP, value: *mut ::core::ffi::c_void, valuesize: u32, index: *mut u32, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsReceiveMessage(::core::mem::transmute(channel), ::core::mem::transmute(message), ::core::mem::transmute(::windows::core::as_ptr_or_null(messagedescriptions)), messagedescriptions.len() as _, ::core::mem::transmute(receiveoption), ::core::mem::transmute(readbodyoption), ::core::mem::transmute(heap), ::core::mem::transmute(value), ::core::mem::transmute(valuesize), ::core::mem::transmute(index), ::core::mem::transmute(asynccontext), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsRegisterOperationForCancel(context: *const WS_OPERATION_CONTEXT, cancelcallback: WS_OPERATION_CANCEL_CALLBACK, freestatecallback: WS_OPERATION_FREE_STATE_CALLBACK, userstate: *const ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsRegisterOperationForCancel(context: *const WS_OPERATION_CONTEXT, cancelcallback: *mut ::core::ffi::c_void, freestatecallback: *mut ::core::ffi::c_void, userstate: *const ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsRegisterOperationForCancel(::core::mem::transmute(context), ::core::mem::transmute(cancelcallback), ::core::mem::transmute(freestatecallback), ::core::mem::transmute(userstate), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsRemoveCustomHeader(message: *const WS_MESSAGE, headername: *const WS_XML_STRING, headerns: *const WS_XML_STRING, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsRemoveCustomHeader(message: *const WS_MESSAGE, headername: *const WS_XML_STRING, headerns: *const WS_XML_STRING, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsRemoveCustomHeader(::core::mem::transmute(message), ::core::mem::transmute(headername), ::core::mem::transmute(headerns), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsRemoveHeader(message: *const WS_MESSAGE, headertype: WS_HEADER_TYPE, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsRemoveHeader(message: *const WS_MESSAGE, headertype: WS_HEADER_TYPE, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsRemoveHeader(::core::mem::transmute(message), ::core::mem::transmute(headertype), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsRemoveMappedHeader(message: *const WS_MESSAGE, headername: *const WS_XML_STRING, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsRemoveMappedHeader(message: *const WS_MESSAGE, headername: *const WS_XML_STRING, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsRemoveMappedHeader(::core::mem::transmute(message), ::core::mem::transmute(headername), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsRemoveNode(nodeposition: *const WS_XML_NODE_POSITION, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsRemoveNode(nodeposition: *const WS_XML_NODE_POSITION, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsRemoveNode(::core::mem::transmute(nodeposition), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsRequestReply(channel: *const WS_CHANNEL, requestmessage: *const WS_MESSAGE, requestmessagedescription: *const WS_MESSAGE_DESCRIPTION, writeoption: WS_WRITE_OPTION, requestbodyvalue: *const ::core::ffi::c_void, requestbodyvaluesize: u32, replymessage: *const WS_MESSAGE, replymessagedescription: *const WS_MESSAGE_DESCRIPTION, readoption: WS_READ_OPTION, heap: *const WS_HEAP, value: *mut ::core::ffi::c_void, valuesize: u32, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsRequestReply(channel: *const WS_CHANNEL, requestmessage: *const WS_MESSAGE, requestmessagedescription: *const WS_MESSAGE_DESCRIPTION, writeoption: WS_WRITE_OPTION, requestbodyvalue: *const ::core::ffi::c_void, requestbodyvaluesize: u32, replymessage: *const WS_MESSAGE, replymessagedescription: *const WS_MESSAGE_DESCRIPTION, readoption: WS_READ_OPTION, heap: *const WS_HEAP, value: *mut ::core::ffi::c_void, valuesize: u32, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsRequestReply(
            ::core::mem::transmute(channel),
            ::core::mem::transmute(requestmessage),
            ::core::mem::transmute(requestmessagedescription),
            ::core::mem::transmute(writeoption),
            ::core::mem::transmute(requestbodyvalue),
            ::core::mem::transmute(requestbodyvaluesize),
            ::core::mem::transmute(replymessage),
            ::core::mem::transmute(replymessagedescription),
            ::core::mem::transmute(readoption),
            ::core::mem::transmute(heap),
            ::core::mem::transmute(value),
            ::core::mem::transmute(valuesize),
            ::core::mem::transmute(asynccontext),
            ::core::mem::transmute(error),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsRequestSecurityToken(channel: *const WS_CHANNEL, properties: &[WS_REQUEST_SECURITY_TOKEN_PROPERTY], token: *mut *mut WS_SECURITY_TOKEN, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsRequestSecurityToken(channel: *const WS_CHANNEL, properties: *const WS_REQUEST_SECURITY_TOKEN_PROPERTY, propertycount: u32, token: *mut *mut WS_SECURITY_TOKEN, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsRequestSecurityToken(::core::mem::transmute(channel), ::core::mem::transmute(::windows::core::as_ptr_or_null(properties)), properties.len() as _, ::core::mem::transmute(token), ::core::mem::transmute(asynccontext), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsResetChannel(channel: *const WS_CHANNEL, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsResetChannel(channel: *const WS_CHANNEL, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsResetChannel(::core::mem::transmute(channel), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsResetError(error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsResetError(error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsResetError(::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsResetHeap(heap: *const WS_HEAP, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsResetHeap(heap: *const WS_HEAP, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsResetHeap(::core::mem::transmute(heap), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsResetListener(listener: *const WS_LISTENER, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsResetListener(listener: *const WS_LISTENER, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsResetListener(::core::mem::transmute(listener), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsResetMessage(message: *const WS_MESSAGE, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsResetMessage(message: *const WS_MESSAGE, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsResetMessage(::core::mem::transmute(message), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsResetMetadata(metadata: *const WS_METADATA, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsResetMetadata(metadata: *const WS_METADATA, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsResetMetadata(::core::mem::transmute(metadata), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsResetServiceHost(servicehost: *const WS_SERVICE_HOST, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsResetServiceHost(servicehost: *const WS_SERVICE_HOST, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsResetServiceHost(::core::mem::transmute(servicehost), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsResetServiceProxy(serviceproxy: *const WS_SERVICE_PROXY, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsResetServiceProxy(serviceproxy: *const WS_SERVICE_PROXY, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsResetServiceProxy(::core::mem::transmute(serviceproxy), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsRevokeSecurityContext(securitycontext: *const WS_SECURITY_CONTEXT, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsRevokeSecurityContext(securitycontext: *const WS_SECURITY_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsRevokeSecurityContext(::core::mem::transmute(securitycontext), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsSendFaultMessageForError(channel: *const WS_CHANNEL, replymessage: *const WS_MESSAGE, faulterror: *const WS_ERROR, faulterrorcode: ::windows::core::HRESULT, faultdisclosure: WS_FAULT_DISCLOSURE, requestmessage: *const WS_MESSAGE, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsSendFaultMessageForError(channel: *const WS_CHANNEL, replymessage: *const WS_MESSAGE, faulterror: *const WS_ERROR, faulterrorcode: ::windows::core::HRESULT, faultdisclosure: WS_FAULT_DISCLOSURE, requestmessage: *const WS_MESSAGE, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsSendFaultMessageForError(::core::mem::transmute(channel), ::core::mem::transmute(replymessage), ::core::mem::transmute(faulterror), ::core::mem::transmute(faulterrorcode), ::core::mem::transmute(faultdisclosure), ::core::mem::transmute(requestmessage), ::core::mem::transmute(asynccontext), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsSendMessage(channel: *const WS_CHANNEL, message: *const WS_MESSAGE, messagedescription: *const WS_MESSAGE_DESCRIPTION, writeoption: WS_WRITE_OPTION, bodyvalue: *const ::core::ffi::c_void, bodyvaluesize: u32, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsSendMessage(channel: *const WS_CHANNEL, message: *const WS_MESSAGE, messagedescription: *const WS_MESSAGE_DESCRIPTION, writeoption: WS_WRITE_OPTION, bodyvalue: *const ::core::ffi::c_void, bodyvaluesize: u32, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsSendMessage(::core::mem::transmute(channel), ::core::mem::transmute(message), ::core::mem::transmute(messagedescription), ::core::mem::transmute(writeoption), ::core::mem::transmute(bodyvalue), ::core::mem::transmute(bodyvaluesize), ::core::mem::transmute(asynccontext), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsSendReplyMessage(channel: *const WS_CHANNEL, replymessage: *const WS_MESSAGE, replymessagedescription: *const WS_MESSAGE_DESCRIPTION, writeoption: WS_WRITE_OPTION, replybodyvalue: *const ::core::ffi::c_void, replybodyvaluesize: u32, requestmessage: *const WS_MESSAGE, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsSendReplyMessage(channel: *const WS_CHANNEL, replymessage: *const WS_MESSAGE, replymessagedescription: *const WS_MESSAGE_DESCRIPTION, writeoption: WS_WRITE_OPTION, replybodyvalue: *const ::core::ffi::c_void, replybodyvaluesize: u32, requestmessage: *const WS_MESSAGE, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsSendReplyMessage(::core::mem::transmute(channel), ::core::mem::transmute(replymessage), ::core::mem::transmute(replymessagedescription), ::core::mem::transmute(writeoption), ::core::mem::transmute(replybodyvalue), ::core::mem::transmute(replybodyvaluesize), ::core::mem::transmute(requestmessage), ::core::mem::transmute(asynccontext), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsSetChannelProperty(channel: *const WS_CHANNEL, id: WS_CHANNEL_PROPERTY_ID, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsSetChannelProperty(channel: *const WS_CHANNEL, id: WS_CHANNEL_PROPERTY_ID, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsSetChannelProperty(::core::mem::transmute(channel), ::core::mem::transmute(id), ::core::mem::transmute(value), ::core::mem::transmute(valuesize), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsSetErrorProperty(error: *const WS_ERROR, id: WS_ERROR_PROPERTY_ID, value: *const ::core::ffi::c_void, valuesize: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsSetErrorProperty(error: *const WS_ERROR, id: WS_ERROR_PROPERTY_ID, value: *const ::core::ffi::c_void, valuesize: u32) -> ::windows::core::HRESULT;
        }
        WsSetErrorProperty(::core::mem::transmute(error), ::core::mem::transmute(id), ::core::mem::transmute(value), ::core::mem::transmute(valuesize)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsSetFaultErrorDetail(error: *const WS_ERROR, faultdetaildescription: *const WS_FAULT_DETAIL_DESCRIPTION, writeoption: WS_WRITE_OPTION, value: *const ::core::ffi::c_void, valuesize: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsSetFaultErrorDetail(error: *const WS_ERROR, faultdetaildescription: *const WS_FAULT_DETAIL_DESCRIPTION, writeoption: WS_WRITE_OPTION, value: *const ::core::ffi::c_void, valuesize: u32) -> ::windows::core::HRESULT;
        }
        WsSetFaultErrorDetail(::core::mem::transmute(error), ::core::mem::transmute(faultdetaildescription), ::core::mem::transmute(writeoption), ::core::mem::transmute(value), ::core::mem::transmute(valuesize)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsSetFaultErrorProperty(error: *const WS_ERROR, id: WS_FAULT_ERROR_PROPERTY_ID, value: *const ::core::ffi::c_void, valuesize: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsSetFaultErrorProperty(error: *const WS_ERROR, id: WS_FAULT_ERROR_PROPERTY_ID, value: *const ::core::ffi::c_void, valuesize: u32) -> ::windows::core::HRESULT;
        }
        WsSetFaultErrorProperty(::core::mem::transmute(error), ::core::mem::transmute(id), ::core::mem::transmute(value), ::core::mem::transmute(valuesize)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsSetHeader(message: *const WS_MESSAGE, headertype: WS_HEADER_TYPE, valuetype: WS_TYPE, writeoption: WS_WRITE_OPTION, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsSetHeader(message: *const WS_MESSAGE, headertype: WS_HEADER_TYPE, valuetype: WS_TYPE, writeoption: WS_WRITE_OPTION, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsSetHeader(::core::mem::transmute(message), ::core::mem::transmute(headertype), ::core::mem::transmute(valuetype), ::core::mem::transmute(writeoption), ::core::mem::transmute(value), ::core::mem::transmute(valuesize), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsSetInput(reader: *const WS_XML_READER, encoding: *const WS_XML_READER_ENCODING, input: *const WS_XML_READER_INPUT, properties: &[WS_XML_READER_PROPERTY], error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsSetInput(reader: *const WS_XML_READER, encoding: *const WS_XML_READER_ENCODING, input: *const WS_XML_READER_INPUT, properties: *const WS_XML_READER_PROPERTY, propertycount: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsSetInput(::core::mem::transmute(reader), ::core::mem::transmute(encoding), ::core::mem::transmute(input), ::core::mem::transmute(::windows::core::as_ptr_or_null(properties)), properties.len() as _, ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsSetInputToBuffer(reader: *const WS_XML_READER, buffer: *const WS_XML_BUFFER, properties: &[WS_XML_READER_PROPERTY], error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsSetInputToBuffer(reader: *const WS_XML_READER, buffer: *const WS_XML_BUFFER, properties: *const WS_XML_READER_PROPERTY, propertycount: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsSetInputToBuffer(::core::mem::transmute(reader), ::core::mem::transmute(buffer), ::core::mem::transmute(::windows::core::as_ptr_or_null(properties)), properties.len() as _, ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsSetListenerProperty(listener: *const WS_LISTENER, id: WS_LISTENER_PROPERTY_ID, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsSetListenerProperty(listener: *const WS_LISTENER, id: WS_LISTENER_PROPERTY_ID, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsSetListenerProperty(::core::mem::transmute(listener), ::core::mem::transmute(id), ::core::mem::transmute(value), ::core::mem::transmute(valuesize), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsSetMessageProperty(message: *const WS_MESSAGE, id: WS_MESSAGE_PROPERTY_ID, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsSetMessageProperty(message: *const WS_MESSAGE, id: WS_MESSAGE_PROPERTY_ID, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsSetMessageProperty(::core::mem::transmute(message), ::core::mem::transmute(id), ::core::mem::transmute(value), ::core::mem::transmute(valuesize), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsSetOutput(writer: *const WS_XML_WRITER, encoding: *const WS_XML_WRITER_ENCODING, output: *const WS_XML_WRITER_OUTPUT, properties: &[WS_XML_WRITER_PROPERTY], error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsSetOutput(writer: *const WS_XML_WRITER, encoding: *const WS_XML_WRITER_ENCODING, output: *const WS_XML_WRITER_OUTPUT, properties: *const WS_XML_WRITER_PROPERTY, propertycount: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsSetOutput(::core::mem::transmute(writer), ::core::mem::transmute(encoding), ::core::mem::transmute(output), ::core::mem::transmute(::windows::core::as_ptr_or_null(properties)), properties.len() as _, ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsSetOutputToBuffer(writer: *const WS_XML_WRITER, buffer: *const WS_XML_BUFFER, properties: &[WS_XML_WRITER_PROPERTY], error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsSetOutputToBuffer(writer: *const WS_XML_WRITER, buffer: *const WS_XML_BUFFER, properties: *const WS_XML_WRITER_PROPERTY, propertycount: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsSetOutputToBuffer(::core::mem::transmute(writer), ::core::mem::transmute(buffer), ::core::mem::transmute(::windows::core::as_ptr_or_null(properties)), properties.len() as _, ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsSetReaderPosition(reader: *const WS_XML_READER, nodeposition: *const WS_XML_NODE_POSITION, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsSetReaderPosition(reader: *const WS_XML_READER, nodeposition: *const WS_XML_NODE_POSITION, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsSetReaderPosition(::core::mem::transmute(reader), ::core::mem::transmute(nodeposition), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsSetWriterPosition(writer: *const WS_XML_WRITER, nodeposition: *const WS_XML_NODE_POSITION, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsSetWriterPosition(writer: *const WS_XML_WRITER, nodeposition: *const WS_XML_NODE_POSITION, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsSetWriterPosition(::core::mem::transmute(writer), ::core::mem::transmute(nodeposition), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsShutdownSessionChannel(channel: *const WS_CHANNEL, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsShutdownSessionChannel(channel: *const WS_CHANNEL, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsShutdownSessionChannel(::core::mem::transmute(channel), ::core::mem::transmute(asynccontext), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsSkipNode(reader: *const WS_XML_READER, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsSkipNode(reader: *const WS_XML_READER, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsSkipNode(::core::mem::transmute(reader), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsStartReaderCanonicalization(reader: *const WS_XML_READER, writecallback: WS_WRITE_CALLBACK, writecallbackstate: *const ::core::ffi::c_void, properties: &[WS_XML_CANONICALIZATION_PROPERTY], error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsStartReaderCanonicalization(reader: *const WS_XML_READER, writecallback: *mut ::core::ffi::c_void, writecallbackstate: *const ::core::ffi::c_void, properties: *const WS_XML_CANONICALIZATION_PROPERTY, propertycount: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsStartReaderCanonicalization(::core::mem::transmute(reader), ::core::mem::transmute(writecallback), ::core::mem::transmute(writecallbackstate), ::core::mem::transmute(::windows::core::as_ptr_or_null(properties)), properties.len() as _, ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsStartWriterCanonicalization(writer: *const WS_XML_WRITER, writecallback: WS_WRITE_CALLBACK, writecallbackstate: *const ::core::ffi::c_void, properties: &[WS_XML_CANONICALIZATION_PROPERTY], error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsStartWriterCanonicalization(writer: *const WS_XML_WRITER, writecallback: *mut ::core::ffi::c_void, writecallbackstate: *const ::core::ffi::c_void, properties: *const WS_XML_CANONICALIZATION_PROPERTY, propertycount: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsStartWriterCanonicalization(::core::mem::transmute(writer), ::core::mem::transmute(writecallback), ::core::mem::transmute(writecallbackstate), ::core::mem::transmute(::windows::core::as_ptr_or_null(properties)), properties.len() as _, ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsTrimXmlWhitespace(chars: &[u16], trimmedchars: *mut *mut u16, trimmedcount: *mut u32, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsTrimXmlWhitespace(chars: ::windows::core::PCWSTR, charcount: u32, trimmedchars: *mut *mut u16, trimmedcount: *mut u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsTrimXmlWhitespace(::core::mem::transmute(::windows::core::as_ptr_or_null(chars)), chars.len() as _, ::core::mem::transmute(trimmedchars), ::core::mem::transmute(trimmedcount), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsVerifyXmlNCName(ncnamechars: &[u16], error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsVerifyXmlNCName(ncnamechars: ::windows::core::PCWSTR, ncnamecharcount: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsVerifyXmlNCName(::core::mem::transmute(::windows::core::as_ptr_or_null(ncnamechars)), ncnamechars.len() as _, ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsWriteArray(writer: *const WS_XML_WRITER, localname: *const WS_XML_STRING, ns: *const WS_XML_STRING, valuetype: WS_VALUE_TYPE, array: *const ::core::ffi::c_void, arraysize: u32, itemoffset: u32, itemcount: u32, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsWriteArray(writer: *const WS_XML_WRITER, localname: *const WS_XML_STRING, ns: *const WS_XML_STRING, valuetype: WS_VALUE_TYPE, array: *const ::core::ffi::c_void, arraysize: u32, itemoffset: u32, itemcount: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsWriteArray(::core::mem::transmute(writer), ::core::mem::transmute(localname), ::core::mem::transmute(ns), ::core::mem::transmute(valuetype), ::core::mem::transmute(array), ::core::mem::transmute(arraysize), ::core::mem::transmute(itemoffset), ::core::mem::transmute(itemcount), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsWriteAttribute(writer: *const WS_XML_WRITER, attributedescription: *const WS_ATTRIBUTE_DESCRIPTION, writeoption: WS_WRITE_OPTION, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsWriteAttribute(writer: *const WS_XML_WRITER, attributedescription: *const WS_ATTRIBUTE_DESCRIPTION, writeoption: WS_WRITE_OPTION, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsWriteAttribute(::core::mem::transmute(writer), ::core::mem::transmute(attributedescription), ::core::mem::transmute(writeoption), ::core::mem::transmute(value), ::core::mem::transmute(valuesize), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsWriteBody(message: *const WS_MESSAGE, bodydescription: *const WS_ELEMENT_DESCRIPTION, writeoption: WS_WRITE_OPTION, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsWriteBody(message: *const WS_MESSAGE, bodydescription: *const WS_ELEMENT_DESCRIPTION, writeoption: WS_WRITE_OPTION, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsWriteBody(::core::mem::transmute(message), ::core::mem::transmute(bodydescription), ::core::mem::transmute(writeoption), ::core::mem::transmute(value), ::core::mem::transmute(valuesize), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsWriteBytes(writer: *const WS_XML_WRITER, bytes: *const ::core::ffi::c_void, bytecount: u32, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsWriteBytes(writer: *const WS_XML_WRITER, bytes: *const ::core::ffi::c_void, bytecount: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsWriteBytes(::core::mem::transmute(writer), ::core::mem::transmute(bytes), ::core::mem::transmute(bytecount), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsWriteChars(writer: *const WS_XML_WRITER, chars: &[u16], error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsWriteChars(writer: *const WS_XML_WRITER, chars: ::windows::core::PCWSTR, charcount: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsWriteChars(::core::mem::transmute(writer), ::core::mem::transmute(::windows::core::as_ptr_or_null(chars)), chars.len() as _, ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsWriteCharsUtf8(writer: *const WS_XML_WRITER, bytes: &[u8], error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsWriteCharsUtf8(writer: *const WS_XML_WRITER, bytes: *const u8, bytecount: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsWriteCharsUtf8(::core::mem::transmute(writer), ::core::mem::transmute(::windows::core::as_ptr_or_null(bytes)), bytes.len() as _, ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsWriteElement(writer: *const WS_XML_WRITER, elementdescription: *const WS_ELEMENT_DESCRIPTION, writeoption: WS_WRITE_OPTION, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsWriteElement(writer: *const WS_XML_WRITER, elementdescription: *const WS_ELEMENT_DESCRIPTION, writeoption: WS_WRITE_OPTION, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsWriteElement(::core::mem::transmute(writer), ::core::mem::transmute(elementdescription), ::core::mem::transmute(writeoption), ::core::mem::transmute(value), ::core::mem::transmute(valuesize), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsWriteEndAttribute(writer: *const WS_XML_WRITER, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsWriteEndAttribute(writer: *const WS_XML_WRITER, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsWriteEndAttribute(::core::mem::transmute(writer), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsWriteEndCData(writer: *const WS_XML_WRITER, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsWriteEndCData(writer: *const WS_XML_WRITER, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsWriteEndCData(::core::mem::transmute(writer), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsWriteEndElement(writer: *const WS_XML_WRITER, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsWriteEndElement(writer: *const WS_XML_WRITER, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsWriteEndElement(::core::mem::transmute(writer), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsWriteEndStartElement(writer: *const WS_XML_WRITER, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsWriteEndStartElement(writer: *const WS_XML_WRITER, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsWriteEndStartElement(::core::mem::transmute(writer), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsWriteEnvelopeEnd(message: *const WS_MESSAGE, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsWriteEnvelopeEnd(message: *const WS_MESSAGE, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsWriteEnvelopeEnd(::core::mem::transmute(message), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsWriteEnvelopeStart(message: *const WS_MESSAGE, writer: *const WS_XML_WRITER, donecallback: WS_MESSAGE_DONE_CALLBACK, donecallbackstate: *const ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsWriteEnvelopeStart(message: *const WS_MESSAGE, writer: *const WS_XML_WRITER, donecallback: *mut ::core::ffi::c_void, donecallbackstate: *const ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsWriteEnvelopeStart(::core::mem::transmute(message), ::core::mem::transmute(writer), ::core::mem::transmute(donecallback), ::core::mem::transmute(donecallbackstate), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsWriteMessageEnd(channel: *const WS_CHANNEL, message: *const WS_MESSAGE, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsWriteMessageEnd(channel: *const WS_CHANNEL, message: *const WS_MESSAGE, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsWriteMessageEnd(::core::mem::transmute(channel), ::core::mem::transmute(message), ::core::mem::transmute(asynccontext), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsWriteMessageStart(channel: *const WS_CHANNEL, message: *const WS_MESSAGE, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsWriteMessageStart(channel: *const WS_CHANNEL, message: *const WS_MESSAGE, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsWriteMessageStart(::core::mem::transmute(channel), ::core::mem::transmute(message), ::core::mem::transmute(asynccontext), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsWriteNode(writer: *const WS_XML_WRITER, node: *const WS_XML_NODE, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsWriteNode(writer: *const WS_XML_WRITER, node: *const WS_XML_NODE, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsWriteNode(::core::mem::transmute(writer), ::core::mem::transmute(node), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsWriteQualifiedName(writer: *const WS_XML_WRITER, prefix: *const WS_XML_STRING, localname: *const WS_XML_STRING, ns: *const WS_XML_STRING, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsWriteQualifiedName(writer: *const WS_XML_WRITER, prefix: *const WS_XML_STRING, localname: *const WS_XML_STRING, ns: *const WS_XML_STRING, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsWriteQualifiedName(::core::mem::transmute(writer), ::core::mem::transmute(prefix), ::core::mem::transmute(localname), ::core::mem::transmute(ns), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsWriteStartAttribute<'a, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(writer: *const WS_XML_WRITER, prefix: *const WS_XML_STRING, localname: *const WS_XML_STRING, ns: *const WS_XML_STRING, singlequote: Param4, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsWriteStartAttribute(writer: *const WS_XML_WRITER, prefix: *const WS_XML_STRING, localname: *const WS_XML_STRING, ns: *const WS_XML_STRING, singlequote: super::super::Foundation::BOOL, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsWriteStartAttribute(::core::mem::transmute(writer), ::core::mem::transmute(prefix), ::core::mem::transmute(localname), ::core::mem::transmute(ns), singlequote.into_param().abi(), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsWriteStartCData(writer: *const WS_XML_WRITER, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsWriteStartCData(writer: *const WS_XML_WRITER, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsWriteStartCData(::core::mem::transmute(writer), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsWriteStartElement(writer: *const WS_XML_WRITER, prefix: *const WS_XML_STRING, localname: *const WS_XML_STRING, ns: *const WS_XML_STRING, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsWriteStartElement(writer: *const WS_XML_WRITER, prefix: *const WS_XML_STRING, localname: *const WS_XML_STRING, ns: *const WS_XML_STRING, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsWriteStartElement(::core::mem::transmute(writer), ::core::mem::transmute(prefix), ::core::mem::transmute(localname), ::core::mem::transmute(ns), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsWriteText(writer: *const WS_XML_WRITER, text: *const WS_XML_TEXT, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsWriteText(writer: *const WS_XML_WRITER, text: *const WS_XML_TEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsWriteText(::core::mem::transmute(writer), ::core::mem::transmute(text), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsWriteType(writer: *const WS_XML_WRITER, typemapping: WS_TYPE_MAPPING, r#type: WS_TYPE, typedescription: *const ::core::ffi::c_void, writeoption: WS_WRITE_OPTION, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsWriteType(writer: *const WS_XML_WRITER, typemapping: WS_TYPE_MAPPING, r#type: WS_TYPE, typedescription: *const ::core::ffi::c_void, writeoption: WS_WRITE_OPTION, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsWriteType(::core::mem::transmute(writer), ::core::mem::transmute(typemapping), ::core::mem::transmute(r#type), ::core::mem::transmute(typedescription), ::core::mem::transmute(writeoption), ::core::mem::transmute(value), ::core::mem::transmute(valuesize), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsWriteValue(writer: *const WS_XML_WRITER, valuetype: WS_VALUE_TYPE, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsWriteValue(writer: *const WS_XML_WRITER, valuetype: WS_VALUE_TYPE, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsWriteValue(::core::mem::transmute(writer), ::core::mem::transmute(valuetype), ::core::mem::transmute(value), ::core::mem::transmute(valuesize), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsWriteXmlBuffer(writer: *const WS_XML_WRITER, xmlbuffer: *const WS_XML_BUFFER, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsWriteXmlBuffer(writer: *const WS_XML_WRITER, xmlbuffer: *const WS_XML_BUFFER, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsWriteXmlBuffer(::core::mem::transmute(writer), ::core::mem::transmute(xmlbuffer), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsWriteXmlBufferToBytes(writer: *const WS_XML_WRITER, xmlbuffer: *const WS_XML_BUFFER, encoding: *const WS_XML_WRITER_ENCODING, properties: &[WS_XML_WRITER_PROPERTY], heap: *const WS_HEAP, bytes: *mut *mut ::core::ffi::c_void, bytecount: *mut u32, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsWriteXmlBufferToBytes(writer: *const WS_XML_WRITER, xmlbuffer: *const WS_XML_BUFFER, encoding: *const WS_XML_WRITER_ENCODING, properties: *const WS_XML_WRITER_PROPERTY, propertycount: u32, heap: *const WS_HEAP, bytes: *mut *mut ::core::ffi::c_void, bytecount: *mut u32, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsWriteXmlBufferToBytes(::core::mem::transmute(writer), ::core::mem::transmute(xmlbuffer), ::core::mem::transmute(encoding), ::core::mem::transmute(::windows::core::as_ptr_or_null(properties)), properties.len() as _, ::core::mem::transmute(heap), ::core::mem::transmute(bytes), ::core::mem::transmute(bytecount), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsWriteXmlnsAttribute<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(writer: *const WS_XML_WRITER, prefix: *const WS_XML_STRING, ns: *const WS_XML_STRING, singlequote: Param3, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsWriteXmlnsAttribute(writer: *const WS_XML_WRITER, prefix: *const WS_XML_STRING, ns: *const WS_XML_STRING, singlequote: super::super::Foundation::BOOL, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsWriteXmlnsAttribute(::core::mem::transmute(writer), ::core::mem::transmute(prefix), ::core::mem::transmute(ns), singlequote.into_param().abi(), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsXmlStringEquals(string1: *const WS_XML_STRING, string2: *const WS_XML_STRING, error: *const WS_ERROR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WsXmlStringEquals(string1: *const WS_XML_STRING, string2: *const WS_XML_STRING, error: *const WS_ERROR) -> ::windows::core::HRESULT;
        }
        WsXmlStringEquals(::core::mem::transmute(string1), ::core::mem::transmute(string2), ::core::mem::transmute(error)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
