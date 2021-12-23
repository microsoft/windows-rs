#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub struct BINARY_BLOB_CREDENTIAL_INFO {
    pub cbBlob: u32,
    pub pbBlob: *mut u8,
}
impl ::core::marker::Copy for BINARY_BLOB_CREDENTIAL_INFO {}
impl ::core::clone::Clone for BINARY_BLOB_CREDENTIAL_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BINARY_BLOB_CREDENTIAL_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BINARY_BLOB_CREDENTIAL_INFO").field("cbBlob", &self.cbBlob).field("pbBlob", &self.pbBlob).finish()
    }
}
unsafe impl ::windows::core::Abi for BINARY_BLOB_CREDENTIAL_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BINARY_BLOB_CREDENTIAL_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BINARY_BLOB_CREDENTIAL_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for BINARY_BLOB_CREDENTIAL_INFO {}
impl ::core::default::Default for BINARY_BLOB_CREDENTIAL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub struct CERT_CREDENTIAL_INFO {
    pub cbSize: u32,
    pub rgbHashOfCert: [u8; 20],
}
impl ::core::marker::Copy for CERT_CREDENTIAL_INFO {}
impl ::core::clone::Clone for CERT_CREDENTIAL_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CERT_CREDENTIAL_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_CREDENTIAL_INFO").field("cbSize", &self.cbSize).field("rgbHashOfCert", &self.rgbHashOfCert).finish()
    }
}
unsafe impl ::windows::core::Abi for CERT_CREDENTIAL_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CERT_CREDENTIAL_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CERT_CREDENTIAL_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for CERT_CREDENTIAL_INFO {}
impl ::core::default::Default for CERT_CREDENTIAL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CERT_HASH_LENGTH: u32 = 20u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CREDENTIALA {
    pub Flags: CRED_FLAGS,
    pub Type: CRED_TYPE,
    pub TargetName: super::super::Foundation::PSTR,
    pub Comment: super::super::Foundation::PSTR,
    pub LastWritten: super::super::Foundation::FILETIME,
    pub CredentialBlobSize: u32,
    pub CredentialBlob: *mut u8,
    pub Persist: CRED_PERSIST,
    pub AttributeCount: u32,
    pub Attributes: *mut CREDENTIAL_ATTRIBUTEA,
    pub TargetAlias: super::super::Foundation::PSTR,
    pub UserName: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CREDENTIALA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CREDENTIALA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CREDENTIALA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREDENTIALA")
            .field("Flags", &self.Flags)
            .field("Type", &self.Type)
            .field("TargetName", &self.TargetName)
            .field("Comment", &self.Comment)
            .field("LastWritten", &self.LastWritten)
            .field("CredentialBlobSize", &self.CredentialBlobSize)
            .field("CredentialBlob", &self.CredentialBlob)
            .field("Persist", &self.Persist)
            .field("AttributeCount", &self.AttributeCount)
            .field("Attributes", &self.Attributes)
            .field("TargetAlias", &self.TargetAlias)
            .field("UserName", &self.UserName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CREDENTIALA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CREDENTIALA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CREDENTIALA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CREDENTIALA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CREDENTIALA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CREDENTIALW {
    pub Flags: CRED_FLAGS,
    pub Type: CRED_TYPE,
    pub TargetName: super::super::Foundation::PWSTR,
    pub Comment: super::super::Foundation::PWSTR,
    pub LastWritten: super::super::Foundation::FILETIME,
    pub CredentialBlobSize: u32,
    pub CredentialBlob: *mut u8,
    pub Persist: CRED_PERSIST,
    pub AttributeCount: u32,
    pub Attributes: *mut CREDENTIAL_ATTRIBUTEW,
    pub TargetAlias: super::super::Foundation::PWSTR,
    pub UserName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CREDENTIALW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CREDENTIALW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CREDENTIALW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREDENTIALW")
            .field("Flags", &self.Flags)
            .field("Type", &self.Type)
            .field("TargetName", &self.TargetName)
            .field("Comment", &self.Comment)
            .field("LastWritten", &self.LastWritten)
            .field("CredentialBlobSize", &self.CredentialBlobSize)
            .field("CredentialBlob", &self.CredentialBlob)
            .field("Persist", &self.Persist)
            .field("AttributeCount", &self.AttributeCount)
            .field("Attributes", &self.Attributes)
            .field("TargetAlias", &self.TargetAlias)
            .field("UserName", &self.UserName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CREDENTIALW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CREDENTIALW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CREDENTIALW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CREDENTIALW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CREDENTIALW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CREDENTIAL_ATTRIBUTEA {
    pub Keyword: super::super::Foundation::PSTR,
    pub Flags: u32,
    pub ValueSize: u32,
    pub Value: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CREDENTIAL_ATTRIBUTEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CREDENTIAL_ATTRIBUTEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CREDENTIAL_ATTRIBUTEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREDENTIAL_ATTRIBUTEA").field("Keyword", &self.Keyword).field("Flags", &self.Flags).field("ValueSize", &self.ValueSize).field("Value", &self.Value).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CREDENTIAL_ATTRIBUTEA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CREDENTIAL_ATTRIBUTEA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CREDENTIAL_ATTRIBUTEA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CREDENTIAL_ATTRIBUTEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CREDENTIAL_ATTRIBUTEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CREDENTIAL_ATTRIBUTEW {
    pub Keyword: super::super::Foundation::PWSTR,
    pub Flags: u32,
    pub ValueSize: u32,
    pub Value: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CREDENTIAL_ATTRIBUTEW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CREDENTIAL_ATTRIBUTEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CREDENTIAL_ATTRIBUTEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREDENTIAL_ATTRIBUTEW").field("Keyword", &self.Keyword).field("Flags", &self.Flags).field("ValueSize", &self.ValueSize).field("Value", &self.Value).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CREDENTIAL_ATTRIBUTEW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CREDENTIAL_ATTRIBUTEW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CREDENTIAL_ATTRIBUTEW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CREDENTIAL_ATTRIBUTEW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CREDENTIAL_ATTRIBUTEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CREDENTIAL_TARGET_INFORMATIONA {
    pub TargetName: super::super::Foundation::PSTR,
    pub NetbiosServerName: super::super::Foundation::PSTR,
    pub DnsServerName: super::super::Foundation::PSTR,
    pub NetbiosDomainName: super::super::Foundation::PSTR,
    pub DnsDomainName: super::super::Foundation::PSTR,
    pub DnsTreeName: super::super::Foundation::PSTR,
    pub PackageName: super::super::Foundation::PSTR,
    pub Flags: u32,
    pub CredTypeCount: u32,
    pub CredTypes: *mut u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CREDENTIAL_TARGET_INFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CREDENTIAL_TARGET_INFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CREDENTIAL_TARGET_INFORMATIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREDENTIAL_TARGET_INFORMATIONA").field("TargetName", &self.TargetName).field("NetbiosServerName", &self.NetbiosServerName).field("DnsServerName", &self.DnsServerName).field("NetbiosDomainName", &self.NetbiosDomainName).field("DnsDomainName", &self.DnsDomainName).field("DnsTreeName", &self.DnsTreeName).field("PackageName", &self.PackageName).field("Flags", &self.Flags).field("CredTypeCount", &self.CredTypeCount).field("CredTypes", &self.CredTypes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CREDENTIAL_TARGET_INFORMATIONA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CREDENTIAL_TARGET_INFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CREDENTIAL_TARGET_INFORMATIONA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CREDENTIAL_TARGET_INFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CREDENTIAL_TARGET_INFORMATIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CREDENTIAL_TARGET_INFORMATIONW {
    pub TargetName: super::super::Foundation::PWSTR,
    pub NetbiosServerName: super::super::Foundation::PWSTR,
    pub DnsServerName: super::super::Foundation::PWSTR,
    pub NetbiosDomainName: super::super::Foundation::PWSTR,
    pub DnsDomainName: super::super::Foundation::PWSTR,
    pub DnsTreeName: super::super::Foundation::PWSTR,
    pub PackageName: super::super::Foundation::PWSTR,
    pub Flags: u32,
    pub CredTypeCount: u32,
    pub CredTypes: *mut u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CREDENTIAL_TARGET_INFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CREDENTIAL_TARGET_INFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CREDENTIAL_TARGET_INFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREDENTIAL_TARGET_INFORMATIONW").field("TargetName", &self.TargetName).field("NetbiosServerName", &self.NetbiosServerName).field("DnsServerName", &self.DnsServerName).field("NetbiosDomainName", &self.NetbiosDomainName).field("DnsDomainName", &self.DnsDomainName).field("DnsTreeName", &self.DnsTreeName).field("PackageName", &self.PackageName).field("Flags", &self.Flags).field("CredTypeCount", &self.CredTypeCount).field("CredTypes", &self.CredTypes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CREDENTIAL_TARGET_INFORMATIONW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CREDENTIAL_TARGET_INFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CREDENTIAL_TARGET_INFORMATIONW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CREDENTIAL_TARGET_INFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CREDENTIAL_TARGET_INFORMATIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub type CREDSPP_SUBMIT_TYPE = i32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CredsspPasswordCreds: CREDSPP_SUBMIT_TYPE = 2i32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CredsspSchannelCreds: CREDSPP_SUBMIT_TYPE = 4i32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CredsspCertificateCreds: CREDSPP_SUBMIT_TYPE = 13i32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CredsspSubmitBufferBoth: CREDSPP_SUBMIT_TYPE = 50i32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CredsspSubmitBufferBothOld: CREDSPP_SUBMIT_TYPE = 51i32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CredsspCredEx: CREDSPP_SUBMIT_TYPE = 100i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub struct CREDSSP_CRED {
    pub Type: CREDSPP_SUBMIT_TYPE,
    pub pSchannelCred: *mut ::core::ffi::c_void,
    pub pSpnegoCred: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for CREDSSP_CRED {}
impl ::core::clone::Clone for CREDSSP_CRED {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CREDSSP_CRED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREDSSP_CRED").field("Type", &self.Type).field("pSchannelCred", &self.pSchannelCred).field("pSpnegoCred", &self.pSpnegoCred).finish()
    }
}
unsafe impl ::windows::core::Abi for CREDSSP_CRED {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CREDSSP_CRED {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CREDSSP_CRED>()) == 0 }
    }
}
impl ::core::cmp::Eq for CREDSSP_CRED {}
impl ::core::default::Default for CREDSSP_CRED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub struct CREDSSP_CRED_EX {
    pub Type: CREDSPP_SUBMIT_TYPE,
    pub Version: u32,
    pub Flags: u32,
    pub Reserved: u32,
    pub Cred: CREDSSP_CRED,
}
impl ::core::marker::Copy for CREDSSP_CRED_EX {}
impl ::core::clone::Clone for CREDSSP_CRED_EX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CREDSSP_CRED_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREDSSP_CRED_EX").field("Type", &self.Type).field("Version", &self.Version).field("Flags", &self.Flags).field("Reserved", &self.Reserved).field("Cred", &self.Cred).finish()
    }
}
unsafe impl ::windows::core::Abi for CREDSSP_CRED_EX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CREDSSP_CRED_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CREDSSP_CRED_EX>()) == 0 }
    }
}
impl ::core::cmp::Eq for CREDSSP_CRED_EX {}
impl ::core::default::Default for CREDSSP_CRED_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CREDSSP_CRED_EX_VERSION: u32 = 0u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CREDSSP_FLAG_REDIRECT: u32 = 1u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CREDSSP_SERVER_AUTH_CERTIFICATE: u32 = 2u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CREDSSP_SERVER_AUTH_LOOPBACK: u32 = 4u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CREDSSP_SERVER_AUTH_NEGOTIATE: u32 = 1u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CREDUIWIN_DOWNLEVEL_HELLO_AS_SMART_CARD: u32 = 2147483648u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub type CREDUIWIN_FLAGS = u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CREDUIWIN_GENERIC: CREDUIWIN_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CREDUIWIN_CHECKBOX: CREDUIWIN_FLAGS = 2u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CREDUIWIN_AUTHPACKAGE_ONLY: CREDUIWIN_FLAGS = 16u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CREDUIWIN_IN_CRED_ONLY: CREDUIWIN_FLAGS = 32u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CREDUIWIN_ENUMERATE_ADMINS: CREDUIWIN_FLAGS = 256u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CREDUIWIN_ENUMERATE_CURRENT_USER: CREDUIWIN_FLAGS = 512u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CREDUIWIN_SECURE_PROMPT: CREDUIWIN_FLAGS = 4096u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CREDUIWIN_PREPROMPTING: CREDUIWIN_FLAGS = 8192u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CREDUIWIN_PACK_32_WOW: CREDUIWIN_FLAGS = 268435456u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CREDUIWIN_IGNORE_CLOUDAUTHORITY_NAME: u32 = 262144u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub type CREDUI_FLAGS = u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CREDUI_FLAGS_ALWAYS_SHOW_UI: CREDUI_FLAGS = 128u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CREDUI_FLAGS_COMPLETE_USERNAME: CREDUI_FLAGS = 2048u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CREDUI_FLAGS_DO_NOT_PERSIST: CREDUI_FLAGS = 2u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CREDUI_FLAGS_EXCLUDE_CERTIFICATES: CREDUI_FLAGS = 8u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CREDUI_FLAGS_EXPECT_CONFIRMATION: CREDUI_FLAGS = 131072u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CREDUI_FLAGS_GENERIC_CREDENTIALS: CREDUI_FLAGS = 262144u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CREDUI_FLAGS_INCORRECT_PASSWORD: CREDUI_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CREDUI_FLAGS_KEEP_USERNAME: CREDUI_FLAGS = 1048576u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CREDUI_FLAGS_PASSWORD_ONLY_OK: CREDUI_FLAGS = 512u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CREDUI_FLAGS_PERSIST: CREDUI_FLAGS = 4096u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CREDUI_FLAGS_REQUEST_ADMINISTRATOR: CREDUI_FLAGS = 4u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CREDUI_FLAGS_REQUIRE_CERTIFICATE: CREDUI_FLAGS = 16u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CREDUI_FLAGS_REQUIRE_SMARTCARD: CREDUI_FLAGS = 256u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CREDUI_FLAGS_SERVER_CREDENTIAL: CREDUI_FLAGS = 16384u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CREDUI_FLAGS_SHOW_SAVE_CHECK_BOX: CREDUI_FLAGS = 64u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CREDUI_FLAGS_USERNAME_TARGET_CREDENTIALS: CREDUI_FLAGS = 524288u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CREDUI_FLAGS_VALIDATE_USERNAME: CREDUI_FLAGS = 1024u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation', 'Win32_Graphics_Gdi'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct CREDUI_INFOA {
    pub cbSize: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub pszMessageText: super::super::Foundation::PSTR,
    pub pszCaptionText: super::super::Foundation::PSTR,
    pub hbmBanner: super::super::Graphics::Gdi::HBITMAP,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for CREDUI_INFOA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for CREDUI_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for CREDUI_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREDUI_INFOA").field("cbSize", &self.cbSize).field("hwndParent", &self.hwndParent).field("pszMessageText", &self.pszMessageText).field("pszCaptionText", &self.pszCaptionText).field("hbmBanner", &self.hbmBanner).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for CREDUI_INFOA {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for CREDUI_INFOA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CREDUI_INFOA>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for CREDUI_INFOA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for CREDUI_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation', 'Win32_Graphics_Gdi'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct CREDUI_INFOW {
    pub cbSize: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub pszMessageText: super::super::Foundation::PWSTR,
    pub pszCaptionText: super::super::Foundation::PWSTR,
    pub hbmBanner: super::super::Graphics::Gdi::HBITMAP,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for CREDUI_INFOW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for CREDUI_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for CREDUI_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREDUI_INFOW").field("cbSize", &self.cbSize).field("hwndParent", &self.hwndParent).field("pszMessageText", &self.pszMessageText).field("pszCaptionText", &self.pszCaptionText).field("hbmBanner", &self.hbmBanner).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for CREDUI_INFOW {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for CREDUI_INFOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CREDUI_INFOW>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for CREDUI_INFOW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for CREDUI_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CREDUI_MAX_CAPTION_LENGTH: u32 = 128u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CREDUI_MAX_GENERIC_TARGET_LENGTH: u32 = 32767u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CREDUI_MAX_MESSAGE_LENGTH: u32 = 1024u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_ALLOW_NAME_RESOLUTION: u32 = 1u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_CACHE_TARGET_INFORMATION: u32 = 1u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub type CRED_ENUMERATE_FLAGS = u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_ENUMERATE_ALL_CREDENTIALS: CRED_ENUMERATE_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub type CRED_FLAGS = u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_FLAGS_PASSWORD_FOR_CERT: CRED_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_FLAGS_PROMPT_NOW: CRED_FLAGS = 2u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_FLAGS_USERNAME_TARGET: CRED_FLAGS = 4u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_FLAGS_OWF_CRED_BLOB: CRED_FLAGS = 8u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_FLAGS_REQUIRE_CONFIRMATION: CRED_FLAGS = 16u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_FLAGS_WILDCARD_MATCH: CRED_FLAGS = 32u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_FLAGS_VSM_PROTECTED: CRED_FLAGS = 64u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_FLAGS_NGC_CERT: CRED_FLAGS = 128u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_FLAGS_VALID_FLAGS: CRED_FLAGS = 61695u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_FLAGS_VALID_INPUT_FLAGS: CRED_FLAGS = 61599u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_LOGON_TYPES_MASK: u32 = 61440u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub type CRED_MARSHAL_TYPE = i32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CertCredential: CRED_MARSHAL_TYPE = 1i32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const UsernameTargetCredential: CRED_MARSHAL_TYPE = 2i32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const BinaryBlobCredential: CRED_MARSHAL_TYPE = 3i32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const UsernameForPackedCredentials: CRED_MARSHAL_TYPE = 4i32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const BinaryBlobForSystem: CRED_MARSHAL_TYPE = 5i32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_MAX_ATTRIBUTES: u32 = 64u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_MAX_GENERIC_TARGET_NAME_LENGTH: u32 = 32767u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_MAX_STRING_LENGTH: u32 = 256u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_MAX_TARGETNAME_ATTRIBUTE_LENGTH: u32 = 256u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_MAX_TARGETNAME_NAMESPACE_LENGTH: u32 = 256u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_MAX_VALUE_SIZE: u32 = 256u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub type CRED_PACK_FLAGS = u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_PACK_PROTECTED_CREDENTIALS: CRED_PACK_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_PACK_WOW_BUFFER: CRED_PACK_FLAGS = 2u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_PACK_GENERIC_CREDENTIALS: CRED_PACK_FLAGS = 4u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_PACK_ID_PROVIDER_CREDENTIALS: CRED_PACK_FLAGS = 8u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub type CRED_PERSIST = u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_PERSIST_NONE: CRED_PERSIST = 0u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_PERSIST_SESSION: CRED_PERSIST = 1u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_PERSIST_LOCAL_MACHINE: CRED_PERSIST = 2u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_PERSIST_ENTERPRISE: CRED_PERSIST = 3u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_PRESERVE_CREDENTIAL_BLOB: u32 = 1u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub type CRED_PROTECTION_TYPE = i32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CredUnprotected: CRED_PROTECTION_TYPE = 0i32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CredUserProtection: CRED_PROTECTION_TYPE = 1i32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CredTrustedProtection: CRED_PROTECTION_TYPE = 2i32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CredForSystemProtection: CRED_PROTECTION_TYPE = 3i32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_PROTECT_AS_SELF: u32 = 1u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_PROTECT_TO_SYSTEM: u32 = 2u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_TI_CREATE_EXPLICIT_CRED: u32 = 16u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_TI_DNSTREE_IS_DFS_SERVER: u32 = 64u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_TI_DOMAIN_FORMAT_UNKNOWN: u32 = 2u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_TI_ONLY_PASSWORD_REQUIRED: u32 = 4u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_TI_SERVER_FORMAT_UNKNOWN: u32 = 1u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_TI_USERNAME_TARGET: u32 = 8u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_TI_VALID_FLAGS: u32 = 61567u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_TI_WORKGROUP_MEMBER: u32 = 32u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub type CRED_TYPE = u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_TYPE_GENERIC: CRED_TYPE = 1u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_TYPE_DOMAIN_PASSWORD: CRED_TYPE = 2u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_TYPE_DOMAIN_CERTIFICATE: CRED_TYPE = 3u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_TYPE_DOMAIN_VISIBLE_PASSWORD: CRED_TYPE = 4u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_TYPE_GENERIC_CERTIFICATE: CRED_TYPE = 5u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_TYPE_DOMAIN_EXTENDED: CRED_TYPE = 6u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_TYPE_MAXIMUM: CRED_TYPE = 7u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_TYPE_MAXIMUM_EX: CRED_TYPE = 1007u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_UNPROTECT_ALLOW_TO_SYSTEM: u32 = 2u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const CRED_UNPROTECT_AS_SELF: u32 = 1u32;
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredDeleteA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(targetname: Param0, r#type: u32, flags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredDeleteA(targetname: super::super::Foundation::PSTR, r#type: u32, flags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CredDeleteA(targetname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredDeleteW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(targetname: Param0, r#type: u32, flags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredDeleteW(targetname: super::super::Foundation::PWSTR, r#type: u32, flags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CredDeleteW(targetname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredEnumerateA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(filter: Param0, flags: CRED_ENUMERATE_FLAGS, count: *mut u32, credential: *mut *mut *mut CREDENTIALA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredEnumerateA(filter: super::super::Foundation::PSTR, flags: CRED_ENUMERATE_FLAGS, count: *mut u32, credential: *mut *mut *mut CREDENTIALA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CredEnumerateA(filter.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(count), ::core::mem::transmute(credential)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredEnumerateW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(filter: Param0, flags: CRED_ENUMERATE_FLAGS, count: *mut u32, credential: *mut *mut *mut CREDENTIALW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredEnumerateW(filter: super::super::Foundation::PWSTR, flags: CRED_ENUMERATE_FLAGS, count: *mut u32, credential: *mut *mut *mut CREDENTIALW) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CredEnumerateW(filter.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(count), ::core::mem::transmute(credential)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredFindBestCredentialA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(targetname: Param0, r#type: u32, flags: u32, credential: *mut *mut CREDENTIALA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredFindBestCredentialA(targetname: super::super::Foundation::PSTR, r#type: u32, flags: u32, credential: *mut *mut CREDENTIALA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CredFindBestCredentialA(targetname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(flags), ::core::mem::transmute(credential)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredFindBestCredentialW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(targetname: Param0, r#type: u32, flags: u32, credential: *mut *mut CREDENTIALW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredFindBestCredentialW(targetname: super::super::Foundation::PWSTR, r#type: u32, flags: u32, credential: *mut *mut CREDENTIALW) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CredFindBestCredentialW(targetname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(flags), ::core::mem::transmute(credential)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
#[inline]
pub unsafe fn CredFree(buffer: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredFree(buffer: *const ::core::ffi::c_void);
        }
        CredFree(::core::mem::transmute(buffer))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredGetSessionTypes(maximumpersistcount: u32, maximumpersist: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredGetSessionTypes(maximumpersistcount: u32, maximumpersist: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CredGetSessionTypes(::core::mem::transmute(maximumpersistcount), ::core::mem::transmute(maximumpersist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredGetTargetInfoA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(targetname: Param0, flags: u32, targetinfo: *mut *mut CREDENTIAL_TARGET_INFORMATIONA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredGetTargetInfoA(targetname: super::super::Foundation::PSTR, flags: u32, targetinfo: *mut *mut CREDENTIAL_TARGET_INFORMATIONA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CredGetTargetInfoA(targetname.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(targetinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredGetTargetInfoW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(targetname: Param0, flags: u32, targetinfo: *mut *mut CREDENTIAL_TARGET_INFORMATIONW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredGetTargetInfoW(targetname: super::super::Foundation::PWSTR, flags: u32, targetinfo: *mut *mut CREDENTIAL_TARGET_INFORMATIONW) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CredGetTargetInfoW(targetname.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(targetinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredIsMarshaledCredentialA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(marshaledcredential: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredIsMarshaledCredentialA(marshaledcredential: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CredIsMarshaledCredentialA(marshaledcredential.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredIsMarshaledCredentialW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(marshaledcredential: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredIsMarshaledCredentialW(marshaledcredential: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CredIsMarshaledCredentialW(marshaledcredential.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredIsProtectedA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(pszprotectedcredentials: Param0, pprotectiontype: *mut CRED_PROTECTION_TYPE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredIsProtectedA(pszprotectedcredentials: super::super::Foundation::PSTR, pprotectiontype: *mut CRED_PROTECTION_TYPE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CredIsProtectedA(pszprotectedcredentials.into_param().abi(), ::core::mem::transmute(pprotectiontype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredIsProtectedW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pszprotectedcredentials: Param0, pprotectiontype: *mut CRED_PROTECTION_TYPE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredIsProtectedW(pszprotectedcredentials: super::super::Foundation::PWSTR, pprotectiontype: *mut CRED_PROTECTION_TYPE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CredIsProtectedW(pszprotectedcredentials.into_param().abi(), ::core::mem::transmute(pprotectiontype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredMarshalCredentialA(credtype: CRED_MARSHAL_TYPE, credential: *const ::core::ffi::c_void, marshaledcredential: *mut super::super::Foundation::PSTR) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredMarshalCredentialA(credtype: CRED_MARSHAL_TYPE, credential: *const ::core::ffi::c_void, marshaledcredential: *mut super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CredMarshalCredentialA(::core::mem::transmute(credtype), ::core::mem::transmute(credential), ::core::mem::transmute(marshaledcredential)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredMarshalCredentialW(credtype: CRED_MARSHAL_TYPE, credential: *const ::core::ffi::c_void, marshaledcredential: *mut super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredMarshalCredentialW(credtype: CRED_MARSHAL_TYPE, credential: *const ::core::ffi::c_void, marshaledcredential: *mut super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CredMarshalCredentialW(::core::mem::transmute(credtype), ::core::mem::transmute(credential), ::core::mem::transmute(marshaledcredential)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredPackAuthenticationBufferA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(dwflags: CRED_PACK_FLAGS, pszusername: Param1, pszpassword: Param2, ppackedcredentials: *mut u8, pcbpackedcredentials: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredPackAuthenticationBufferA(dwflags: CRED_PACK_FLAGS, pszusername: super::super::Foundation::PSTR, pszpassword: super::super::Foundation::PSTR, ppackedcredentials: *mut u8, pcbpackedcredentials: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CredPackAuthenticationBufferA(::core::mem::transmute(dwflags), pszusername.into_param().abi(), pszpassword.into_param().abi(), ::core::mem::transmute(ppackedcredentials), ::core::mem::transmute(pcbpackedcredentials)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredPackAuthenticationBufferW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(dwflags: CRED_PACK_FLAGS, pszusername: Param1, pszpassword: Param2, ppackedcredentials: *mut u8, pcbpackedcredentials: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredPackAuthenticationBufferW(dwflags: CRED_PACK_FLAGS, pszusername: super::super::Foundation::PWSTR, pszpassword: super::super::Foundation::PWSTR, ppackedcredentials: *mut u8, pcbpackedcredentials: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CredPackAuthenticationBufferW(::core::mem::transmute(dwflags), pszusername.into_param().abi(), pszpassword.into_param().abi(), ::core::mem::transmute(ppackedcredentials), ::core::mem::transmute(pcbpackedcredentials)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredProtectA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(fasself: Param0, pszcredentials: Param1, cchcredentials: u32, pszprotectedcredentials: super::super::Foundation::PSTR, pcchmaxchars: *mut u32, protectiontype: *mut CRED_PROTECTION_TYPE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredProtectA(fasself: super::super::Foundation::BOOL, pszcredentials: super::super::Foundation::PSTR, cchcredentials: u32, pszprotectedcredentials: super::super::Foundation::PSTR, pcchmaxchars: *mut u32, protectiontype: *mut CRED_PROTECTION_TYPE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CredProtectA(fasself.into_param().abi(), pszcredentials.into_param().abi(), ::core::mem::transmute(cchcredentials), ::core::mem::transmute(pszprotectedcredentials), ::core::mem::transmute(pcchmaxchars), ::core::mem::transmute(protectiontype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredProtectW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(fasself: Param0, pszcredentials: Param1, cchcredentials: u32, pszprotectedcredentials: super::super::Foundation::PWSTR, pcchmaxchars: *mut u32, protectiontype: *mut CRED_PROTECTION_TYPE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredProtectW(fasself: super::super::Foundation::BOOL, pszcredentials: super::super::Foundation::PWSTR, cchcredentials: u32, pszprotectedcredentials: super::super::Foundation::PWSTR, pcchmaxchars: *mut u32, protectiontype: *mut CRED_PROTECTION_TYPE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CredProtectW(fasself.into_param().abi(), pszcredentials.into_param().abi(), ::core::mem::transmute(cchcredentials), ::core::mem::transmute(pszprotectedcredentials), ::core::mem::transmute(pcchmaxchars), ::core::mem::transmute(protectiontype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredReadA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(targetname: Param0, r#type: u32, flags: u32, credential: *mut *mut CREDENTIALA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredReadA(targetname: super::super::Foundation::PSTR, r#type: u32, flags: u32, credential: *mut *mut CREDENTIALA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CredReadA(targetname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(flags), ::core::mem::transmute(credential)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredReadDomainCredentialsA(targetinfo: *const CREDENTIAL_TARGET_INFORMATIONA, flags: u32, count: *mut u32, credential: *mut *mut *mut CREDENTIALA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredReadDomainCredentialsA(targetinfo: *const CREDENTIAL_TARGET_INFORMATIONA, flags: u32, count: *mut u32, credential: *mut *mut *mut CREDENTIALA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CredReadDomainCredentialsA(::core::mem::transmute(targetinfo), ::core::mem::transmute(flags), ::core::mem::transmute(count), ::core::mem::transmute(credential)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredReadDomainCredentialsW(targetinfo: *const CREDENTIAL_TARGET_INFORMATIONW, flags: u32, count: *mut u32, credential: *mut *mut *mut CREDENTIALW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredReadDomainCredentialsW(targetinfo: *const CREDENTIAL_TARGET_INFORMATIONW, flags: u32, count: *mut u32, credential: *mut *mut *mut CREDENTIALW) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CredReadDomainCredentialsW(::core::mem::transmute(targetinfo), ::core::mem::transmute(flags), ::core::mem::transmute(count), ::core::mem::transmute(credential)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredReadW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(targetname: Param0, r#type: u32, flags: u32, credential: *mut *mut CREDENTIALW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredReadW(targetname: super::super::Foundation::PWSTR, r#type: u32, flags: u32, credential: *mut *mut CREDENTIALW) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CredReadW(targetname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(flags), ::core::mem::transmute(credential)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredRenameA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(oldtargetname: Param0, newtargetname: Param1, r#type: u32, flags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredRenameA(oldtargetname: super::super::Foundation::PSTR, newtargetname: super::super::Foundation::PSTR, r#type: u32, flags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CredRenameA(oldtargetname.into_param().abi(), newtargetname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredRenameW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(oldtargetname: Param0, newtargetname: Param1, r#type: u32, flags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredRenameW(oldtargetname: super::super::Foundation::PWSTR, newtargetname: super::super::Foundation::PWSTR, r#type: u32, flags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CredRenameW(oldtargetname.into_param().abi(), newtargetname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredUICmdLinePromptForCredentialsA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(psztargetname: Param0, pcontext: *mut SecHandle, dwautherror: u32, username: Param3, uluserbuffersize: u32, pszpassword: Param5, ulpasswordbuffersize: u32, pfsave: *mut super::super::Foundation::BOOL, dwflags: CREDUI_FLAGS) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredUICmdLinePromptForCredentialsA(psztargetname: super::super::Foundation::PSTR, pcontext: *mut SecHandle, dwautherror: u32, username: super::super::Foundation::PSTR, uluserbuffersize: u32, pszpassword: super::super::Foundation::PSTR, ulpasswordbuffersize: u32, pfsave: *mut super::super::Foundation::BOOL, dwflags: CREDUI_FLAGS) -> u32;
        }
        ::core::mem::transmute(CredUICmdLinePromptForCredentialsA(psztargetname.into_param().abi(), ::core::mem::transmute(pcontext), ::core::mem::transmute(dwautherror), username.into_param().abi(), ::core::mem::transmute(uluserbuffersize), pszpassword.into_param().abi(), ::core::mem::transmute(ulpasswordbuffersize), ::core::mem::transmute(pfsave), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredUICmdLinePromptForCredentialsW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(psztargetname: Param0, pcontext: *mut SecHandle, dwautherror: u32, username: Param3, uluserbuffersize: u32, pszpassword: Param5, ulpasswordbuffersize: u32, pfsave: *mut super::super::Foundation::BOOL, dwflags: CREDUI_FLAGS) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredUICmdLinePromptForCredentialsW(psztargetname: super::super::Foundation::PWSTR, pcontext: *mut SecHandle, dwautherror: u32, username: super::super::Foundation::PWSTR, uluserbuffersize: u32, pszpassword: super::super::Foundation::PWSTR, ulpasswordbuffersize: u32, pfsave: *mut super::super::Foundation::BOOL, dwflags: CREDUI_FLAGS) -> u32;
        }
        ::core::mem::transmute(CredUICmdLinePromptForCredentialsW(psztargetname.into_param().abi(), ::core::mem::transmute(pcontext), ::core::mem::transmute(dwautherror), username.into_param().abi(), ::core::mem::transmute(uluserbuffersize), pszpassword.into_param().abi(), ::core::mem::transmute(ulpasswordbuffersize), ::core::mem::transmute(pfsave), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredUIConfirmCredentialsA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(psztargetname: Param0, bconfirm: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredUIConfirmCredentialsA(psztargetname: super::super::Foundation::PSTR, bconfirm: super::super::Foundation::BOOL) -> u32;
        }
        ::core::mem::transmute(CredUIConfirmCredentialsA(psztargetname.into_param().abi(), bconfirm.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredUIConfirmCredentialsW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(psztargetname: Param0, bconfirm: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredUIConfirmCredentialsW(psztargetname: super::super::Foundation::PWSTR, bconfirm: super::super::Foundation::BOOL) -> u32;
        }
        ::core::mem::transmute(CredUIConfirmCredentialsW(psztargetname.into_param().abi(), bconfirm.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredUIParseUserNameA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(username: Param0, user: super::super::Foundation::PSTR, userbuffersize: u32, domain: super::super::Foundation::PSTR, domainbuffersize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredUIParseUserNameA(username: super::super::Foundation::PSTR, user: super::super::Foundation::PSTR, userbuffersize: u32, domain: super::super::Foundation::PSTR, domainbuffersize: u32) -> u32;
        }
        ::core::mem::transmute(CredUIParseUserNameA(username.into_param().abi(), ::core::mem::transmute(user), ::core::mem::transmute(userbuffersize), ::core::mem::transmute(domain), ::core::mem::transmute(domainbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredUIParseUserNameW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(username: Param0, user: super::super::Foundation::PWSTR, userbuffersize: u32, domain: super::super::Foundation::PWSTR, domainbuffersize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredUIParseUserNameW(username: super::super::Foundation::PWSTR, user: super::super::Foundation::PWSTR, userbuffersize: u32, domain: super::super::Foundation::PWSTR, domainbuffersize: u32) -> u32;
        }
        ::core::mem::transmute(CredUIParseUserNameW(username.into_param().abi(), ::core::mem::transmute(user), ::core::mem::transmute(userbuffersize), ::core::mem::transmute(domain), ::core::mem::transmute(domainbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation', 'Win32_Graphics_Gdi'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn CredUIPromptForCredentialsA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param6: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(puiinfo: *const CREDUI_INFOA, psztargetname: Param1, pcontext: *mut SecHandle, dwautherror: u32, pszusername: Param4, ulusernamebuffersize: u32, pszpassword: Param6, ulpasswordbuffersize: u32, save: *mut super::super::Foundation::BOOL, dwflags: CREDUI_FLAGS) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredUIPromptForCredentialsA(puiinfo: *const CREDUI_INFOA, psztargetname: super::super::Foundation::PSTR, pcontext: *mut SecHandle, dwautherror: u32, pszusername: super::super::Foundation::PSTR, ulusernamebuffersize: u32, pszpassword: super::super::Foundation::PSTR, ulpasswordbuffersize: u32, save: *mut super::super::Foundation::BOOL, dwflags: CREDUI_FLAGS) -> u32;
        }
        ::core::mem::transmute(CredUIPromptForCredentialsA(::core::mem::transmute(puiinfo), psztargetname.into_param().abi(), ::core::mem::transmute(pcontext), ::core::mem::transmute(dwautherror), pszusername.into_param().abi(), ::core::mem::transmute(ulusernamebuffersize), pszpassword.into_param().abi(), ::core::mem::transmute(ulpasswordbuffersize), ::core::mem::transmute(save), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation', 'Win32_Graphics_Gdi'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn CredUIPromptForCredentialsW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param6: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(puiinfo: *const CREDUI_INFOW, psztargetname: Param1, pcontext: *mut SecHandle, dwautherror: u32, pszusername: Param4, ulusernamebuffersize: u32, pszpassword: Param6, ulpasswordbuffersize: u32, save: *mut super::super::Foundation::BOOL, dwflags: CREDUI_FLAGS) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredUIPromptForCredentialsW(puiinfo: *const CREDUI_INFOW, psztargetname: super::super::Foundation::PWSTR, pcontext: *mut SecHandle, dwautherror: u32, pszusername: super::super::Foundation::PWSTR, ulusernamebuffersize: u32, pszpassword: super::super::Foundation::PWSTR, ulpasswordbuffersize: u32, save: *mut super::super::Foundation::BOOL, dwflags: CREDUI_FLAGS) -> u32;
        }
        ::core::mem::transmute(CredUIPromptForCredentialsW(::core::mem::transmute(puiinfo), psztargetname.into_param().abi(), ::core::mem::transmute(pcontext), ::core::mem::transmute(dwautherror), pszusername.into_param().abi(), ::core::mem::transmute(ulusernamebuffersize), pszpassword.into_param().abi(), ::core::mem::transmute(ulpasswordbuffersize), ::core::mem::transmute(save), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation', 'Win32_Graphics_Gdi'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn CredUIPromptForWindowsCredentialsA(puiinfo: *const CREDUI_INFOA, dwautherror: u32, pulauthpackage: *mut u32, pvinauthbuffer: *const ::core::ffi::c_void, ulinauthbuffersize: u32, ppvoutauthbuffer: *mut *mut ::core::ffi::c_void, puloutauthbuffersize: *mut u32, pfsave: *mut super::super::Foundation::BOOL, dwflags: CREDUIWIN_FLAGS) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredUIPromptForWindowsCredentialsA(puiinfo: *const CREDUI_INFOA, dwautherror: u32, pulauthpackage: *mut u32, pvinauthbuffer: *const ::core::ffi::c_void, ulinauthbuffersize: u32, ppvoutauthbuffer: *mut *mut ::core::ffi::c_void, puloutauthbuffersize: *mut u32, pfsave: *mut super::super::Foundation::BOOL, dwflags: CREDUIWIN_FLAGS) -> u32;
        }
        ::core::mem::transmute(CredUIPromptForWindowsCredentialsA(::core::mem::transmute(puiinfo), ::core::mem::transmute(dwautherror), ::core::mem::transmute(pulauthpackage), ::core::mem::transmute(pvinauthbuffer), ::core::mem::transmute(ulinauthbuffersize), ::core::mem::transmute(ppvoutauthbuffer), ::core::mem::transmute(puloutauthbuffersize), ::core::mem::transmute(pfsave), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation', 'Win32_Graphics_Gdi'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn CredUIPromptForWindowsCredentialsW(puiinfo: *const CREDUI_INFOW, dwautherror: u32, pulauthpackage: *mut u32, pvinauthbuffer: *const ::core::ffi::c_void, ulinauthbuffersize: u32, ppvoutauthbuffer: *mut *mut ::core::ffi::c_void, puloutauthbuffersize: *mut u32, pfsave: *mut super::super::Foundation::BOOL, dwflags: CREDUIWIN_FLAGS) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredUIPromptForWindowsCredentialsW(puiinfo: *const CREDUI_INFOW, dwautherror: u32, pulauthpackage: *mut u32, pvinauthbuffer: *const ::core::ffi::c_void, ulinauthbuffersize: u32, ppvoutauthbuffer: *mut *mut ::core::ffi::c_void, puloutauthbuffersize: *mut u32, pfsave: *mut super::super::Foundation::BOOL, dwflags: CREDUIWIN_FLAGS) -> u32;
        }
        ::core::mem::transmute(CredUIPromptForWindowsCredentialsW(::core::mem::transmute(puiinfo), ::core::mem::transmute(dwautherror), ::core::mem::transmute(pulauthpackage), ::core::mem::transmute(pvinauthbuffer), ::core::mem::transmute(ulinauthbuffersize), ::core::mem::transmute(ppvoutauthbuffer), ::core::mem::transmute(puloutauthbuffersize), ::core::mem::transmute(pfsave), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredUIReadSSOCredW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pszrealm: Param0, ppszusername: *mut super::super::Foundation::PWSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredUIReadSSOCredW(pszrealm: super::super::Foundation::PWSTR, ppszusername: *mut super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(CredUIReadSSOCredW(pszrealm.into_param().abi(), ::core::mem::transmute(ppszusername)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredUIStoreSSOCredW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(pszrealm: Param0, pszusername: Param1, pszpassword: Param2, bpersist: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredUIStoreSSOCredW(pszrealm: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR, pszpassword: super::super::Foundation::PWSTR, bpersist: super::super::Foundation::BOOL) -> u32;
        }
        ::core::mem::transmute(CredUIStoreSSOCredW(pszrealm.into_param().abi(), pszusername.into_param().abi(), pszpassword.into_param().abi(), bpersist.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredUnPackAuthenticationBufferA(dwflags: CRED_PACK_FLAGS, pauthbuffer: *const ::core::ffi::c_void, cbauthbuffer: u32, pszusername: super::super::Foundation::PSTR, pcchlmaxusername: *mut u32, pszdomainname: super::super::Foundation::PSTR, pcchmaxdomainname: *mut u32, pszpassword: super::super::Foundation::PSTR, pcchmaxpassword: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredUnPackAuthenticationBufferA(dwflags: CRED_PACK_FLAGS, pauthbuffer: *const ::core::ffi::c_void, cbauthbuffer: u32, pszusername: super::super::Foundation::PSTR, pcchlmaxusername: *mut u32, pszdomainname: super::super::Foundation::PSTR, pcchmaxdomainname: *mut u32, pszpassword: super::super::Foundation::PSTR, pcchmaxpassword: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CredUnPackAuthenticationBufferA(::core::mem::transmute(dwflags), ::core::mem::transmute(pauthbuffer), ::core::mem::transmute(cbauthbuffer), ::core::mem::transmute(pszusername), ::core::mem::transmute(pcchlmaxusername), ::core::mem::transmute(pszdomainname), ::core::mem::transmute(pcchmaxdomainname), ::core::mem::transmute(pszpassword), ::core::mem::transmute(pcchmaxpassword)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredUnPackAuthenticationBufferW(dwflags: CRED_PACK_FLAGS, pauthbuffer: *const ::core::ffi::c_void, cbauthbuffer: u32, pszusername: super::super::Foundation::PWSTR, pcchmaxusername: *mut u32, pszdomainname: super::super::Foundation::PWSTR, pcchmaxdomainname: *mut u32, pszpassword: super::super::Foundation::PWSTR, pcchmaxpassword: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredUnPackAuthenticationBufferW(dwflags: CRED_PACK_FLAGS, pauthbuffer: *const ::core::ffi::c_void, cbauthbuffer: u32, pszusername: super::super::Foundation::PWSTR, pcchmaxusername: *mut u32, pszdomainname: super::super::Foundation::PWSTR, pcchmaxdomainname: *mut u32, pszpassword: super::super::Foundation::PWSTR, pcchmaxpassword: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CredUnPackAuthenticationBufferW(::core::mem::transmute(dwflags), ::core::mem::transmute(pauthbuffer), ::core::mem::transmute(cbauthbuffer), ::core::mem::transmute(pszusername), ::core::mem::transmute(pcchmaxusername), ::core::mem::transmute(pszdomainname), ::core::mem::transmute(pcchmaxdomainname), ::core::mem::transmute(pszpassword), ::core::mem::transmute(pcchmaxpassword)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredUnmarshalCredentialA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(marshaledcredential: Param0, credtype: *mut CRED_MARSHAL_TYPE, credential: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredUnmarshalCredentialA(marshaledcredential: super::super::Foundation::PSTR, credtype: *mut CRED_MARSHAL_TYPE, credential: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CredUnmarshalCredentialA(marshaledcredential.into_param().abi(), ::core::mem::transmute(credtype), ::core::mem::transmute(credential)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredUnmarshalCredentialW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(marshaledcredential: Param0, credtype: *mut CRED_MARSHAL_TYPE, credential: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredUnmarshalCredentialW(marshaledcredential: super::super::Foundation::PWSTR, credtype: *mut CRED_MARSHAL_TYPE, credential: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CredUnmarshalCredentialW(marshaledcredential.into_param().abi(), ::core::mem::transmute(credtype), ::core::mem::transmute(credential)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredUnprotectA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(fasself: Param0, pszprotectedcredentials: Param1, cchprotectedcredentials: u32, pszcredentials: super::super::Foundation::PSTR, pcchmaxchars: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredUnprotectA(fasself: super::super::Foundation::BOOL, pszprotectedcredentials: super::super::Foundation::PSTR, cchprotectedcredentials: u32, pszcredentials: super::super::Foundation::PSTR, pcchmaxchars: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CredUnprotectA(fasself.into_param().abi(), pszprotectedcredentials.into_param().abi(), ::core::mem::transmute(cchprotectedcredentials), ::core::mem::transmute(pszcredentials), ::core::mem::transmute(pcchmaxchars)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredUnprotectW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(fasself: Param0, pszprotectedcredentials: Param1, cchprotectedcredentials: u32, pszcredentials: super::super::Foundation::PWSTR, pcchmaxchars: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredUnprotectW(fasself: super::super::Foundation::BOOL, pszprotectedcredentials: super::super::Foundation::PWSTR, cchprotectedcredentials: u32, pszcredentials: super::super::Foundation::PWSTR, pcchmaxchars: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CredUnprotectW(fasself.into_param().abi(), pszprotectedcredentials.into_param().abi(), ::core::mem::transmute(cchprotectedcredentials), ::core::mem::transmute(pszcredentials), ::core::mem::transmute(pcchmaxchars)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredWriteA(credential: *const CREDENTIALA, flags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredWriteA(credential: *const CREDENTIALA, flags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CredWriteA(::core::mem::transmute(credential), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredWriteDomainCredentialsA(targetinfo: *const CREDENTIAL_TARGET_INFORMATIONA, credential: *const CREDENTIALA, flags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredWriteDomainCredentialsA(targetinfo: *const CREDENTIAL_TARGET_INFORMATIONA, credential: *const CREDENTIALA, flags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CredWriteDomainCredentialsA(::core::mem::transmute(targetinfo), ::core::mem::transmute(credential), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredWriteDomainCredentialsW(targetinfo: *const CREDENTIAL_TARGET_INFORMATIONW, credential: *const CREDENTIALW, flags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredWriteDomainCredentialsW(targetinfo: *const CREDENTIAL_TARGET_INFORMATIONW, credential: *const CREDENTIALW, flags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CredWriteDomainCredentialsW(::core::mem::transmute(targetinfo), ::core::mem::transmute(credential), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredWriteW(credential: *const CREDENTIALW, flags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredWriteW(credential: *const CREDENTIALW, flags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CredWriteW(::core::mem::transmute(credential), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const FILE_DEVICE_SMARTCARD: u32 = 49u32;
pub const GUID_DEVINTERFACE_SMARTCARD_READER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50dd5230_ba8a_11d1_bf5d_0000f805f530);
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetOpenCardNameA(param0: *mut OPENCARDNAMEA) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetOpenCardNameA(param0: *mut OPENCARDNAMEA) -> i32;
        }
        ::core::mem::transmute(GetOpenCardNameA(::core::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetOpenCardNameW(param0: *mut OPENCARDNAMEW) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetOpenCardNameW(param0: *mut OPENCARDNAMEW) -> i32;
        }
        ::core::mem::transmute(GetOpenCardNameW(::core::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
#[inline]
pub unsafe fn KeyCredentialManagerFreeInformation(keycredentialmanagerinfo: *const KeyCredentialManagerInfo) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn KeyCredentialManagerFreeInformation(keycredentialmanagerinfo: *const KeyCredentialManagerInfo);
        }
        KeyCredentialManagerFreeInformation(::core::mem::transmute(keycredentialmanagerinfo))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
#[inline]
pub unsafe fn KeyCredentialManagerGetInformation() -> ::windows::core::Result<*mut KeyCredentialManagerInfo> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn KeyCredentialManagerGetInformation(keycredentialmanagerinfo: *mut *mut KeyCredentialManagerInfo) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut KeyCredentialManagerInfo = ::core::mem::zeroed();
        KeyCredentialManagerGetInformation(::core::mem::transmute(&mut result__)).from_abi::<*mut KeyCredentialManagerInfo>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn KeyCredentialManagerGetOperationErrorStates(keycredentialmanageroperationtype: KeyCredentialManagerOperationType, isready: *mut super::super::Foundation::BOOL, keycredentialmanageroperationerrorstates: *mut KeyCredentialManagerOperationErrorStates) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn KeyCredentialManagerGetOperationErrorStates(keycredentialmanageroperationtype: KeyCredentialManagerOperationType, isready: *mut super::super::Foundation::BOOL, keycredentialmanageroperationerrorstates: *mut KeyCredentialManagerOperationErrorStates) -> ::windows::core::HRESULT;
        }
        KeyCredentialManagerGetOperationErrorStates(::core::mem::transmute(keycredentialmanageroperationtype), ::core::mem::transmute(isready), ::core::mem::transmute(keycredentialmanageroperationerrorstates)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub struct KeyCredentialManagerInfo {
    pub containerId: ::windows::core::GUID,
}
impl ::core::marker::Copy for KeyCredentialManagerInfo {}
impl ::core::clone::Clone for KeyCredentialManagerInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for KeyCredentialManagerInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KeyCredentialManagerInfo").field("containerId", &self.containerId).finish()
    }
}
unsafe impl ::windows::core::Abi for KeyCredentialManagerInfo {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KeyCredentialManagerInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KeyCredentialManagerInfo>()) == 0 }
    }
}
impl ::core::cmp::Eq for KeyCredentialManagerInfo {}
impl ::core::default::Default for KeyCredentialManagerInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub type KeyCredentialManagerOperationErrorStates = u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const KeyCredentialManagerOperationErrorStateNone: KeyCredentialManagerOperationErrorStates = 0u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const KeyCredentialManagerOperationErrorStateDeviceJoinFailure: KeyCredentialManagerOperationErrorStates = 1u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const KeyCredentialManagerOperationErrorStateTokenFailure: KeyCredentialManagerOperationErrorStates = 2u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const KeyCredentialManagerOperationErrorStateCertificateFailure: KeyCredentialManagerOperationErrorStates = 4u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const KeyCredentialManagerOperationErrorStateRemoteSessionFailure: KeyCredentialManagerOperationErrorStates = 8u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const KeyCredentialManagerOperationErrorStatePolicyFailure: KeyCredentialManagerOperationErrorStates = 16u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const KeyCredentialManagerOperationErrorStateHardwareFailure: KeyCredentialManagerOperationErrorStates = 32u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const KeyCredentialManagerOperationErrorStatePinExistsFailure: KeyCredentialManagerOperationErrorStates = 64u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub type KeyCredentialManagerOperationType = i32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const KeyCredentialManagerProvisioning: KeyCredentialManagerOperationType = 0i32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const KeyCredentialManagerPinChange: KeyCredentialManagerOperationType = 1i32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const KeyCredentialManagerPinReset: KeyCredentialManagerOperationType = 2i32;
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn KeyCredentialManagerShowUIOperation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwndowner: Param0, keycredentialmanageroperationtype: KeyCredentialManagerOperationType) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn KeyCredentialManagerShowUIOperation(hwndowner: super::super::Foundation::HWND, keycredentialmanageroperationtype: KeyCredentialManagerOperationType) -> ::windows::core::HRESULT;
        }
        KeyCredentialManagerShowUIOperation(hwndowner.into_param().abi(), ::core::mem::transmute(keycredentialmanageroperationtype)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPOCNCHKPROC = ::core::option::Option<unsafe extern "system" fn(param0: usize, param1: usize, param2: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPOCNCONNPROCA = ::core::option::Option<unsafe extern "system" fn(param0: usize, param1: super::super::Foundation::PSTR, param2: super::super::Foundation::PSTR, param3: *const ::core::ffi::c_void) -> usize>;
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPOCNCONNPROCW = ::core::option::Option<unsafe extern "system" fn(param0: usize, param1: super::super::Foundation::PWSTR, param2: super::super::Foundation::PWSTR, param3: *const ::core::ffi::c_void) -> usize>;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub type LPOCNDSCPROC = ::core::option::Option<unsafe extern "system" fn(param0: usize, param1: usize, param2: *const ::core::ffi::c_void)>;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const MAXIMUM_ATTR_STRING_LENGTH: u32 = 32u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const MAXIMUM_SMARTCARD_READERS: u32 = 10u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OPENCARDNAMEA {
    pub dwStructSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub hSCardContext: usize,
    pub lpstrGroupNames: super::super::Foundation::PSTR,
    pub nMaxGroupNames: u32,
    pub lpstrCardNames: super::super::Foundation::PSTR,
    pub nMaxCardNames: u32,
    pub rgguidInterfaces: *mut ::windows::core::GUID,
    pub cguidInterfaces: u32,
    pub lpstrRdr: super::super::Foundation::PSTR,
    pub nMaxRdr: u32,
    pub lpstrCard: super::super::Foundation::PSTR,
    pub nMaxCard: u32,
    pub lpstrTitle: super::super::Foundation::PSTR,
    pub dwFlags: u32,
    pub pvUserData: *mut ::core::ffi::c_void,
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
    pub dwActiveProtocol: u32,
    pub lpfnConnect: LPOCNCONNPROCA,
    pub lpfnCheck: LPOCNCHKPROC,
    pub lpfnDisconnect: LPOCNDSCPROC,
    pub hCardHandle: usize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OPENCARDNAMEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OPENCARDNAMEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OPENCARDNAMEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OPENCARDNAMEA")
            .field("dwStructSize", &self.dwStructSize)
            .field("hwndOwner", &self.hwndOwner)
            .field("hSCardContext", &self.hSCardContext)
            .field("lpstrGroupNames", &self.lpstrGroupNames)
            .field("nMaxGroupNames", &self.nMaxGroupNames)
            .field("lpstrCardNames", &self.lpstrCardNames)
            .field("nMaxCardNames", &self.nMaxCardNames)
            .field("rgguidInterfaces", &self.rgguidInterfaces)
            .field("cguidInterfaces", &self.cguidInterfaces)
            .field("lpstrRdr", &self.lpstrRdr)
            .field("nMaxRdr", &self.nMaxRdr)
            .field("lpstrCard", &self.lpstrCard)
            .field("nMaxCard", &self.nMaxCard)
            .field("lpstrTitle", &self.lpstrTitle)
            .field("dwFlags", &self.dwFlags)
            .field("pvUserData", &self.pvUserData)
            .field("dwShareMode", &self.dwShareMode)
            .field("dwPreferredProtocols", &self.dwPreferredProtocols)
            .field("dwActiveProtocol", &self.dwActiveProtocol)
            .field("lpfnConnect", &self.lpfnConnect.map(|f| f as usize))
            .field("lpfnCheck", &self.lpfnCheck.map(|f| f as usize))
            .field("lpfnDisconnect", &self.lpfnDisconnect.map(|f| f as usize))
            .field("hCardHandle", &self.hCardHandle)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OPENCARDNAMEA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OPENCARDNAMEA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OPENCARDNAMEA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OPENCARDNAMEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OPENCARDNAMEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OPENCARDNAMEW {
    pub dwStructSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub hSCardContext: usize,
    pub lpstrGroupNames: super::super::Foundation::PWSTR,
    pub nMaxGroupNames: u32,
    pub lpstrCardNames: super::super::Foundation::PWSTR,
    pub nMaxCardNames: u32,
    pub rgguidInterfaces: *mut ::windows::core::GUID,
    pub cguidInterfaces: u32,
    pub lpstrRdr: super::super::Foundation::PWSTR,
    pub nMaxRdr: u32,
    pub lpstrCard: super::super::Foundation::PWSTR,
    pub nMaxCard: u32,
    pub lpstrTitle: super::super::Foundation::PWSTR,
    pub dwFlags: u32,
    pub pvUserData: *mut ::core::ffi::c_void,
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
    pub dwActiveProtocol: u32,
    pub lpfnConnect: LPOCNCONNPROCW,
    pub lpfnCheck: LPOCNCHKPROC,
    pub lpfnDisconnect: LPOCNDSCPROC,
    pub hCardHandle: usize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OPENCARDNAMEW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OPENCARDNAMEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OPENCARDNAMEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OPENCARDNAMEW")
            .field("dwStructSize", &self.dwStructSize)
            .field("hwndOwner", &self.hwndOwner)
            .field("hSCardContext", &self.hSCardContext)
            .field("lpstrGroupNames", &self.lpstrGroupNames)
            .field("nMaxGroupNames", &self.nMaxGroupNames)
            .field("lpstrCardNames", &self.lpstrCardNames)
            .field("nMaxCardNames", &self.nMaxCardNames)
            .field("rgguidInterfaces", &self.rgguidInterfaces)
            .field("cguidInterfaces", &self.cguidInterfaces)
            .field("lpstrRdr", &self.lpstrRdr)
            .field("nMaxRdr", &self.nMaxRdr)
            .field("lpstrCard", &self.lpstrCard)
            .field("nMaxCard", &self.nMaxCard)
            .field("lpstrTitle", &self.lpstrTitle)
            .field("dwFlags", &self.dwFlags)
            .field("pvUserData", &self.pvUserData)
            .field("dwShareMode", &self.dwShareMode)
            .field("dwPreferredProtocols", &self.dwPreferredProtocols)
            .field("dwActiveProtocol", &self.dwActiveProtocol)
            .field("lpfnConnect", &self.lpfnConnect.map(|f| f as usize))
            .field("lpfnCheck", &self.lpfnCheck.map(|f| f as usize))
            .field("lpfnDisconnect", &self.lpfnDisconnect.map(|f| f as usize))
            .field("hCardHandle", &self.hCardHandle)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OPENCARDNAMEW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OPENCARDNAMEW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OPENCARDNAMEW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OPENCARDNAMEW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OPENCARDNAMEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation', 'Win32_UI_WindowsAndMessaging'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct OPENCARDNAME_EXA {
    pub dwStructSize: u32,
    pub hSCardContext: usize,
    pub hwndOwner: super::super::Foundation::HWND,
    pub dwFlags: u32,
    pub lpstrTitle: super::super::Foundation::PSTR,
    pub lpstrSearchDesc: super::super::Foundation::PSTR,
    pub hIcon: super::super::UI::WindowsAndMessaging::HICON,
    pub pOpenCardSearchCriteria: *mut OPENCARD_SEARCH_CRITERIAA,
    pub lpfnConnect: LPOCNCONNPROCA,
    pub pvUserData: *mut ::core::ffi::c_void,
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
    pub lpstrRdr: super::super::Foundation::PSTR,
    pub nMaxRdr: u32,
    pub lpstrCard: super::super::Foundation::PSTR,
    pub nMaxCard: u32,
    pub dwActiveProtocol: u32,
    pub hCardHandle: usize,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for OPENCARDNAME_EXA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for OPENCARDNAME_EXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::fmt::Debug for OPENCARDNAME_EXA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OPENCARDNAME_EXA")
            .field("dwStructSize", &self.dwStructSize)
            .field("hSCardContext", &self.hSCardContext)
            .field("hwndOwner", &self.hwndOwner)
            .field("dwFlags", &self.dwFlags)
            .field("lpstrTitle", &self.lpstrTitle)
            .field("lpstrSearchDesc", &self.lpstrSearchDesc)
            .field("hIcon", &self.hIcon)
            .field("pOpenCardSearchCriteria", &self.pOpenCardSearchCriteria)
            .field("lpfnConnect", &self.lpfnConnect.map(|f| f as usize))
            .field("pvUserData", &self.pvUserData)
            .field("dwShareMode", &self.dwShareMode)
            .field("dwPreferredProtocols", &self.dwPreferredProtocols)
            .field("lpstrRdr", &self.lpstrRdr)
            .field("nMaxRdr", &self.nMaxRdr)
            .field("lpstrCard", &self.lpstrCard)
            .field("nMaxCard", &self.nMaxCard)
            .field("dwActiveProtocol", &self.dwActiveProtocol)
            .field("hCardHandle", &self.hCardHandle)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for OPENCARDNAME_EXA {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for OPENCARDNAME_EXA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OPENCARDNAME_EXA>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for OPENCARDNAME_EXA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for OPENCARDNAME_EXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation', 'Win32_UI_WindowsAndMessaging'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct OPENCARDNAME_EXW {
    pub dwStructSize: u32,
    pub hSCardContext: usize,
    pub hwndOwner: super::super::Foundation::HWND,
    pub dwFlags: u32,
    pub lpstrTitle: super::super::Foundation::PWSTR,
    pub lpstrSearchDesc: super::super::Foundation::PWSTR,
    pub hIcon: super::super::UI::WindowsAndMessaging::HICON,
    pub pOpenCardSearchCriteria: *mut OPENCARD_SEARCH_CRITERIAW,
    pub lpfnConnect: LPOCNCONNPROCW,
    pub pvUserData: *mut ::core::ffi::c_void,
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
    pub lpstrRdr: super::super::Foundation::PWSTR,
    pub nMaxRdr: u32,
    pub lpstrCard: super::super::Foundation::PWSTR,
    pub nMaxCard: u32,
    pub dwActiveProtocol: u32,
    pub hCardHandle: usize,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for OPENCARDNAME_EXW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for OPENCARDNAME_EXW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::fmt::Debug for OPENCARDNAME_EXW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OPENCARDNAME_EXW")
            .field("dwStructSize", &self.dwStructSize)
            .field("hSCardContext", &self.hSCardContext)
            .field("hwndOwner", &self.hwndOwner)
            .field("dwFlags", &self.dwFlags)
            .field("lpstrTitle", &self.lpstrTitle)
            .field("lpstrSearchDesc", &self.lpstrSearchDesc)
            .field("hIcon", &self.hIcon)
            .field("pOpenCardSearchCriteria", &self.pOpenCardSearchCriteria)
            .field("lpfnConnect", &self.lpfnConnect.map(|f| f as usize))
            .field("pvUserData", &self.pvUserData)
            .field("dwShareMode", &self.dwShareMode)
            .field("dwPreferredProtocols", &self.dwPreferredProtocols)
            .field("lpstrRdr", &self.lpstrRdr)
            .field("nMaxRdr", &self.nMaxRdr)
            .field("lpstrCard", &self.lpstrCard)
            .field("nMaxCard", &self.nMaxCard)
            .field("dwActiveProtocol", &self.dwActiveProtocol)
            .field("hCardHandle", &self.hCardHandle)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for OPENCARDNAME_EXW {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for OPENCARDNAME_EXW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OPENCARDNAME_EXW>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for OPENCARDNAME_EXW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for OPENCARDNAME_EXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OPENCARD_SEARCH_CRITERIAA {
    pub dwStructSize: u32,
    pub lpstrGroupNames: super::super::Foundation::PSTR,
    pub nMaxGroupNames: u32,
    pub rgguidInterfaces: *mut ::windows::core::GUID,
    pub cguidInterfaces: u32,
    pub lpstrCardNames: super::super::Foundation::PSTR,
    pub nMaxCardNames: u32,
    pub lpfnCheck: LPOCNCHKPROC,
    pub lpfnConnect: LPOCNCONNPROCA,
    pub lpfnDisconnect: LPOCNDSCPROC,
    pub pvUserData: *mut ::core::ffi::c_void,
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OPENCARD_SEARCH_CRITERIAA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OPENCARD_SEARCH_CRITERIAA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OPENCARD_SEARCH_CRITERIAA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OPENCARD_SEARCH_CRITERIAA")
            .field("dwStructSize", &self.dwStructSize)
            .field("lpstrGroupNames", &self.lpstrGroupNames)
            .field("nMaxGroupNames", &self.nMaxGroupNames)
            .field("rgguidInterfaces", &self.rgguidInterfaces)
            .field("cguidInterfaces", &self.cguidInterfaces)
            .field("lpstrCardNames", &self.lpstrCardNames)
            .field("nMaxCardNames", &self.nMaxCardNames)
            .field("lpfnCheck", &self.lpfnCheck.map(|f| f as usize))
            .field("lpfnConnect", &self.lpfnConnect.map(|f| f as usize))
            .field("lpfnDisconnect", &self.lpfnDisconnect.map(|f| f as usize))
            .field("pvUserData", &self.pvUserData)
            .field("dwShareMode", &self.dwShareMode)
            .field("dwPreferredProtocols", &self.dwPreferredProtocols)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OPENCARD_SEARCH_CRITERIAA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OPENCARD_SEARCH_CRITERIAA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OPENCARD_SEARCH_CRITERIAA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OPENCARD_SEARCH_CRITERIAA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OPENCARD_SEARCH_CRITERIAA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OPENCARD_SEARCH_CRITERIAW {
    pub dwStructSize: u32,
    pub lpstrGroupNames: super::super::Foundation::PWSTR,
    pub nMaxGroupNames: u32,
    pub rgguidInterfaces: *mut ::windows::core::GUID,
    pub cguidInterfaces: u32,
    pub lpstrCardNames: super::super::Foundation::PWSTR,
    pub nMaxCardNames: u32,
    pub lpfnCheck: LPOCNCHKPROC,
    pub lpfnConnect: LPOCNCONNPROCW,
    pub lpfnDisconnect: LPOCNDSCPROC,
    pub pvUserData: *mut ::core::ffi::c_void,
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OPENCARD_SEARCH_CRITERIAW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OPENCARD_SEARCH_CRITERIAW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OPENCARD_SEARCH_CRITERIAW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OPENCARD_SEARCH_CRITERIAW")
            .field("dwStructSize", &self.dwStructSize)
            .field("lpstrGroupNames", &self.lpstrGroupNames)
            .field("nMaxGroupNames", &self.nMaxGroupNames)
            .field("rgguidInterfaces", &self.rgguidInterfaces)
            .field("cguidInterfaces", &self.cguidInterfaces)
            .field("lpstrCardNames", &self.lpstrCardNames)
            .field("nMaxCardNames", &self.nMaxCardNames)
            .field("lpfnCheck", &self.lpfnCheck.map(|f| f as usize))
            .field("lpfnConnect", &self.lpfnConnect.map(|f| f as usize))
            .field("lpfnDisconnect", &self.lpfnDisconnect.map(|f| f as usize))
            .field("pvUserData", &self.pvUserData)
            .field("dwShareMode", &self.dwShareMode)
            .field("dwPreferredProtocols", &self.dwPreferredProtocols)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OPENCARD_SEARCH_CRITERIAW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OPENCARD_SEARCH_CRITERIAW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OPENCARD_SEARCH_CRITERIAW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OPENCARD_SEARCH_CRITERIAW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OPENCARD_SEARCH_CRITERIAW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub struct READER_SEL_REQUEST {
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
    pub MatchType: READER_SEL_REQUEST_MATCH_TYPE,
    pub Anonymous: READER_SEL_REQUEST_0,
}
impl ::core::marker::Copy for READER_SEL_REQUEST {}
impl ::core::clone::Clone for READER_SEL_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for READER_SEL_REQUEST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for READER_SEL_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<READER_SEL_REQUEST>()) == 0 }
    }
}
impl ::core::cmp::Eq for READER_SEL_REQUEST {}
impl ::core::default::Default for READER_SEL_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub union READER_SEL_REQUEST_0 {
    pub ReaderAndContainerParameter: READER_SEL_REQUEST_0_0,
    pub SerialNumberParameter: READER_SEL_REQUEST_0_1,
}
impl ::core::marker::Copy for READER_SEL_REQUEST_0 {}
impl ::core::clone::Clone for READER_SEL_REQUEST_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for READER_SEL_REQUEST_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for READER_SEL_REQUEST_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<READER_SEL_REQUEST_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for READER_SEL_REQUEST_0 {}
impl ::core::default::Default for READER_SEL_REQUEST_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub struct READER_SEL_REQUEST_0_0 {
    pub cbReaderNameOffset: u32,
    pub cchReaderNameLength: u32,
    pub cbContainerNameOffset: u32,
    pub cchContainerNameLength: u32,
    pub dwDesiredCardModuleVersion: u32,
    pub dwCspFlags: u32,
}
impl ::core::marker::Copy for READER_SEL_REQUEST_0_0 {}
impl ::core::clone::Clone for READER_SEL_REQUEST_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for READER_SEL_REQUEST_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("READER_SEL_REQUEST_0_0").field("cbReaderNameOffset", &self.cbReaderNameOffset).field("cchReaderNameLength", &self.cchReaderNameLength).field("cbContainerNameOffset", &self.cbContainerNameOffset).field("cchContainerNameLength", &self.cchContainerNameLength).field("dwDesiredCardModuleVersion", &self.dwDesiredCardModuleVersion).field("dwCspFlags", &self.dwCspFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for READER_SEL_REQUEST_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for READER_SEL_REQUEST_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<READER_SEL_REQUEST_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for READER_SEL_REQUEST_0_0 {}
impl ::core::default::Default for READER_SEL_REQUEST_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub struct READER_SEL_REQUEST_0_1 {
    pub cbSerialNumberOffset: u32,
    pub cbSerialNumberLength: u32,
    pub dwDesiredCardModuleVersion: u32,
}
impl ::core::marker::Copy for READER_SEL_REQUEST_0_1 {}
impl ::core::clone::Clone for READER_SEL_REQUEST_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for READER_SEL_REQUEST_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("READER_SEL_REQUEST_0_1").field("cbSerialNumberOffset", &self.cbSerialNumberOffset).field("cbSerialNumberLength", &self.cbSerialNumberLength).field("dwDesiredCardModuleVersion", &self.dwDesiredCardModuleVersion).finish()
    }
}
unsafe impl ::windows::core::Abi for READER_SEL_REQUEST_0_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for READER_SEL_REQUEST_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<READER_SEL_REQUEST_0_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for READER_SEL_REQUEST_0_1 {}
impl ::core::default::Default for READER_SEL_REQUEST_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub type READER_SEL_REQUEST_MATCH_TYPE = i32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const RSR_MATCH_TYPE_READER_AND_CONTAINER: READER_SEL_REQUEST_MATCH_TYPE = 1i32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const RSR_MATCH_TYPE_SERIAL_NUMBER: READER_SEL_REQUEST_MATCH_TYPE = 2i32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const RSR_MATCH_TYPE_ALL_CARDS: READER_SEL_REQUEST_MATCH_TYPE = 3i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub struct READER_SEL_RESPONSE {
    pub cbReaderNameOffset: u32,
    pub cchReaderNameLength: u32,
    pub cbCardNameOffset: u32,
    pub cchCardNameLength: u32,
}
impl ::core::marker::Copy for READER_SEL_RESPONSE {}
impl ::core::clone::Clone for READER_SEL_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for READER_SEL_RESPONSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("READER_SEL_RESPONSE").field("cbReaderNameOffset", &self.cbReaderNameOffset).field("cchReaderNameLength", &self.cchReaderNameLength).field("cbCardNameOffset", &self.cbCardNameOffset).field("cchCardNameLength", &self.cchCardNameLength).finish()
    }
}
unsafe impl ::windows::core::Abi for READER_SEL_RESPONSE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for READER_SEL_RESPONSE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<READER_SEL_RESPONSE>()) == 0 }
    }
}
impl ::core::cmp::Eq for READER_SEL_RESPONSE {}
impl ::core::default::Default for READER_SEL_RESPONSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_ABSENT: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub struct SCARD_ATRMASK {
    pub cbAtr: u32,
    pub rgbAtr: [u8; 36],
    pub rgbMask: [u8; 36],
}
impl ::core::marker::Copy for SCARD_ATRMASK {}
impl ::core::clone::Clone for SCARD_ATRMASK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCARD_ATRMASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCARD_ATRMASK").field("cbAtr", &self.cbAtr).field("rgbAtr", &self.rgbAtr).field("rgbMask", &self.rgbMask).finish()
    }
}
unsafe impl ::windows::core::Abi for SCARD_ATRMASK {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCARD_ATRMASK {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCARD_ATRMASK>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCARD_ATRMASK {}
impl ::core::default::Default for SCARD_ATRMASK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_ATR_LENGTH: u32 = 33u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_AUDIT_CHV_FAILURE: u32 = 0u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_AUDIT_CHV_SUCCESS: u32 = 1u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_CLASS_COMMUNICATIONS: u32 = 2u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_CLASS_ICC_STATE: u32 = 9u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_CLASS_IFD_PROTOCOL: u32 = 8u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_CLASS_MECHANICAL: u32 = 6u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_CLASS_PERF: u32 = 32766u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_CLASS_POWER_MGMT: u32 = 4u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_CLASS_PROTOCOL: u32 = 3u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_CLASS_SECURITY: u32 = 5u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_CLASS_SYSTEM: u32 = 32767u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_CLASS_VENDOR_DEFINED: u32 = 7u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_CLASS_VENDOR_INFO: u32 = 1u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_COLD_RESET: u32 = 1u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_EJECT_CARD: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub struct SCARD_IO_REQUEST {
    pub dwProtocol: u32,
    pub cbPciLength: u32,
}
impl ::core::marker::Copy for SCARD_IO_REQUEST {}
impl ::core::clone::Clone for SCARD_IO_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCARD_IO_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCARD_IO_REQUEST").field("dwProtocol", &self.dwProtocol).field("cbPciLength", &self.cbPciLength).finish()
    }
}
unsafe impl ::windows::core::Abi for SCARD_IO_REQUEST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCARD_IO_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCARD_IO_REQUEST>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCARD_IO_REQUEST {}
impl ::core::default::Default for SCARD_IO_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_LEAVE_CARD: u32 = 0u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_NEGOTIABLE: u32 = 5u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_POWERED: u32 = 4u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_POWER_DOWN: u32 = 0u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_PRESENT: u32 = 2u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_PROTOCOL_DEFAULT: u32 = 2147483648u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_PROTOCOL_OPTIMAL: u32 = 0u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_PROTOCOL_RAW: u32 = 65536u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_PROTOCOL_T0: u32 = 1u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_PROTOCOL_T1: u32 = 2u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_PROTOCOL_UNDEFINED: u32 = 0u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_PROVIDER_CSP: u32 = 2u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_PROVIDER_KSP: u32 = 3u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_PROVIDER_PRIMARY: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SCARD_READERSTATEA {
    pub szReader: super::super::Foundation::PSTR,
    pub pvUserData: *mut ::core::ffi::c_void,
    pub dwCurrentState: SCARD_STATE,
    pub dwEventState: SCARD_STATE,
    pub cbAtr: u32,
    pub rgbAtr: [u8; 36],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SCARD_READERSTATEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SCARD_READERSTATEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SCARD_READERSTATEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCARD_READERSTATEA").field("szReader", &self.szReader).field("pvUserData", &self.pvUserData).field("dwCurrentState", &self.dwCurrentState).field("dwEventState", &self.dwEventState).field("cbAtr", &self.cbAtr).field("rgbAtr", &self.rgbAtr).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SCARD_READERSTATEA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SCARD_READERSTATEA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCARD_READERSTATEA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SCARD_READERSTATEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SCARD_READERSTATEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SCARD_READERSTATEW {
    pub szReader: super::super::Foundation::PWSTR,
    pub pvUserData: *mut ::core::ffi::c_void,
    pub dwCurrentState: SCARD_STATE,
    pub dwEventState: SCARD_STATE,
    pub cbAtr: u32,
    pub rgbAtr: [u8; 36],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SCARD_READERSTATEW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SCARD_READERSTATEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SCARD_READERSTATEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCARD_READERSTATEW").field("szReader", &self.szReader).field("pvUserData", &self.pvUserData).field("dwCurrentState", &self.dwCurrentState).field("dwEventState", &self.dwEventState).field("cbAtr", &self.cbAtr).field("rgbAtr", &self.rgbAtr).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SCARD_READERSTATEW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SCARD_READERSTATEW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCARD_READERSTATEW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SCARD_READERSTATEW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SCARD_READERSTATEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_READER_CONFISCATES: u32 = 4u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_READER_CONTACTLESS: u32 = 8u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_READER_EJECTS: u32 = 2u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_READER_SWALLOWS: u32 = 1u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_READER_TYPE_EMBEDDEDSE: u32 = 2048u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_READER_TYPE_IDE: u32 = 16u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_READER_TYPE_KEYBOARD: u32 = 4u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_READER_TYPE_NFC: u32 = 256u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_READER_TYPE_NGC: u32 = 1024u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_READER_TYPE_PARALELL: u32 = 2u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_READER_TYPE_PCMCIA: u32 = 64u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_READER_TYPE_SCSI: u32 = 8u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_READER_TYPE_SERIAL: u32 = 1u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_READER_TYPE_TPM: u32 = 128u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_READER_TYPE_UICC: u32 = 512u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_READER_TYPE_USB: u32 = 32u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_READER_TYPE_VENDOR: u32 = 240u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_RESET_CARD: u32 = 1u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub type SCARD_SCOPE = u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_SCOPE_USER: SCARD_SCOPE = 0u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_SCOPE_SYSTEM: SCARD_SCOPE = 2u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_SCOPE_TERMINAL: u32 = 1u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_SHARE_DIRECT: u32 = 3u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_SHARE_EXCLUSIVE: u32 = 1u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_SHARE_SHARED: u32 = 2u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_SPECIFIC: u32 = 6u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub type SCARD_STATE = u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_STATE_UNAWARE: SCARD_STATE = 0u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_STATE_IGNORE: SCARD_STATE = 1u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_STATE_UNAVAILABLE: SCARD_STATE = 8u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_STATE_EMPTY: SCARD_STATE = 16u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_STATE_PRESENT: SCARD_STATE = 32u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_STATE_ATRMATCH: SCARD_STATE = 64u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_STATE_EXCLUSIVE: SCARD_STATE = 128u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_STATE_INUSE: SCARD_STATE = 256u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_STATE_MUTE: SCARD_STATE = 512u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_STATE_CHANGED: SCARD_STATE = 2u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_STATE_UNKNOWN: SCARD_STATE = 4u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_STATE_UNPOWERED: u32 = 1024u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_SWALLOWED: u32 = 3u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_T0_CMD_LENGTH: u32 = 5u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub struct SCARD_T0_COMMAND {
    pub bCla: u8,
    pub bIns: u8,
    pub bP1: u8,
    pub bP2: u8,
    pub bP3: u8,
}
impl ::core::marker::Copy for SCARD_T0_COMMAND {}
impl ::core::clone::Clone for SCARD_T0_COMMAND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCARD_T0_COMMAND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCARD_T0_COMMAND").field("bCla", &self.bCla).field("bIns", &self.bIns).field("bP1", &self.bP1).field("bP2", &self.bP2).field("bP3", &self.bP3).finish()
    }
}
unsafe impl ::windows::core::Abi for SCARD_T0_COMMAND {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCARD_T0_COMMAND {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCARD_T0_COMMAND>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCARD_T0_COMMAND {}
impl ::core::default::Default for SCARD_T0_COMMAND {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_T0_HEADER_LENGTH: u32 = 7u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub struct SCARD_T0_REQUEST {
    pub ioRequest: SCARD_IO_REQUEST,
    pub bSw1: u8,
    pub bSw2: u8,
    pub Anonymous: SCARD_T0_REQUEST_0,
}
impl ::core::marker::Copy for SCARD_T0_REQUEST {}
impl ::core::clone::Clone for SCARD_T0_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SCARD_T0_REQUEST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCARD_T0_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCARD_T0_REQUEST>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCARD_T0_REQUEST {}
impl ::core::default::Default for SCARD_T0_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub union SCARD_T0_REQUEST_0 {
    pub CmdBytes: SCARD_T0_COMMAND,
    pub rgbHeader: [u8; 5],
}
impl ::core::marker::Copy for SCARD_T0_REQUEST_0 {}
impl ::core::clone::Clone for SCARD_T0_REQUEST_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SCARD_T0_REQUEST_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCARD_T0_REQUEST_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCARD_T0_REQUEST_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCARD_T0_REQUEST_0 {}
impl ::core::default::Default for SCARD_T0_REQUEST_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_T1_EPILOGUE_LENGTH: u32 = 2u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_T1_EPILOGUE_LENGTH_LRC: u32 = 1u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_T1_MAX_IFS: u32 = 254u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_T1_PROLOGUE_LENGTH: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub struct SCARD_T1_REQUEST {
    pub ioRequest: SCARD_IO_REQUEST,
}
impl ::core::marker::Copy for SCARD_T1_REQUEST {}
impl ::core::clone::Clone for SCARD_T1_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCARD_T1_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCARD_T1_REQUEST").field("ioRequest", &self.ioRequest).finish()
    }
}
unsafe impl ::windows::core::Abi for SCARD_T1_REQUEST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCARD_T1_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCARD_T1_REQUEST>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCARD_T1_REQUEST {}
impl ::core::default::Default for SCARD_T1_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_UNPOWER_CARD: u32 = 2u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCARD_WARM_RESET: u32 = 2u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCERR_NOCARDNAME: u32 = 16384u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SCERR_NOGUIDS: u32 = 32768u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SC_DLG_FORCE_UI: u32 = 4u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SC_DLG_MINIMAL_UI: u32 = 1u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SC_DLG_NO_UI: u32 = 2u32;
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardAccessStartedEvent() -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardAccessStartedEvent() -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(SCardAccessStartedEvent())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardAddReaderToGroupA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hcontext: usize, szreadername: Param1, szgroupname: Param2) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardAddReaderToGroupA(hcontext: usize, szreadername: super::super::Foundation::PSTR, szgroupname: super::super::Foundation::PSTR) -> i32;
        }
        ::core::mem::transmute(SCardAddReaderToGroupA(::core::mem::transmute(hcontext), szreadername.into_param().abi(), szgroupname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardAddReaderToGroupW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hcontext: usize, szreadername: Param1, szgroupname: Param2) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardAddReaderToGroupW(hcontext: usize, szreadername: super::super::Foundation::PWSTR, szgroupname: super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(SCardAddReaderToGroupW(::core::mem::transmute(hcontext), szreadername.into_param().abi(), szgroupname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
#[inline]
pub unsafe fn SCardAudit(hcontext: usize, dwevent: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardAudit(hcontext: usize, dwevent: u32) -> i32;
        }
        ::core::mem::transmute(SCardAudit(::core::mem::transmute(hcontext), ::core::mem::transmute(dwevent)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
#[inline]
pub unsafe fn SCardBeginTransaction(hcard: usize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardBeginTransaction(hcard: usize) -> i32;
        }
        ::core::mem::transmute(SCardBeginTransaction(::core::mem::transmute(hcard)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
#[inline]
pub unsafe fn SCardCancel(hcontext: usize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardCancel(hcontext: usize) -> i32;
        }
        ::core::mem::transmute(SCardCancel(::core::mem::transmute(hcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardConnectA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hcontext: usize, szreader: Param1, dwsharemode: u32, dwpreferredprotocols: u32, phcard: *mut usize, pdwactiveprotocol: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardConnectA(hcontext: usize, szreader: super::super::Foundation::PSTR, dwsharemode: u32, dwpreferredprotocols: u32, phcard: *mut usize, pdwactiveprotocol: *mut u32) -> i32;
        }
        ::core::mem::transmute(SCardConnectA(::core::mem::transmute(hcontext), szreader.into_param().abi(), ::core::mem::transmute(dwsharemode), ::core::mem::transmute(dwpreferredprotocols), ::core::mem::transmute(phcard), ::core::mem::transmute(pdwactiveprotocol)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardConnectW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hcontext: usize, szreader: Param1, dwsharemode: u32, dwpreferredprotocols: u32, phcard: *mut usize, pdwactiveprotocol: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardConnectW(hcontext: usize, szreader: super::super::Foundation::PWSTR, dwsharemode: u32, dwpreferredprotocols: u32, phcard: *mut usize, pdwactiveprotocol: *mut u32) -> i32;
        }
        ::core::mem::transmute(SCardConnectW(::core::mem::transmute(hcontext), szreader.into_param().abi(), ::core::mem::transmute(dwsharemode), ::core::mem::transmute(dwpreferredprotocols), ::core::mem::transmute(phcard), ::core::mem::transmute(pdwactiveprotocol)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
#[inline]
pub unsafe fn SCardControl(hcard: usize, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, cbinbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, cboutbuffersize: u32, lpbytesreturned: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardControl(hcard: usize, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, cbinbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, cboutbuffersize: u32, lpbytesreturned: *mut u32) -> i32;
        }
        ::core::mem::transmute(SCardControl(::core::mem::transmute(hcard), ::core::mem::transmute(dwcontrolcode), ::core::mem::transmute(lpinbuffer), ::core::mem::transmute(cbinbuffersize), ::core::mem::transmute(lpoutbuffer), ::core::mem::transmute(cboutbuffersize), ::core::mem::transmute(lpbytesreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
#[inline]
pub unsafe fn SCardDisconnect(hcard: usize, dwdisposition: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardDisconnect(hcard: usize, dwdisposition: u32) -> i32;
        }
        ::core::mem::transmute(SCardDisconnect(::core::mem::transmute(hcard), ::core::mem::transmute(dwdisposition)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
#[inline]
pub unsafe fn SCardDlgExtendedError() -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardDlgExtendedError() -> i32;
        }
        ::core::mem::transmute(SCardDlgExtendedError())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
#[inline]
pub unsafe fn SCardEndTransaction(hcard: usize, dwdisposition: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardEndTransaction(hcard: usize, dwdisposition: u32) -> i32;
        }
        ::core::mem::transmute(SCardEndTransaction(::core::mem::transmute(hcard), ::core::mem::transmute(dwdisposition)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
#[inline]
pub unsafe fn SCardEstablishContext(dwscope: SCARD_SCOPE, pvreserved1: *const ::core::ffi::c_void, pvreserved2: *const ::core::ffi::c_void, phcontext: *mut usize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardEstablishContext(dwscope: SCARD_SCOPE, pvreserved1: *const ::core::ffi::c_void, pvreserved2: *const ::core::ffi::c_void, phcontext: *mut usize) -> i32;
        }
        ::core::mem::transmute(SCardEstablishContext(::core::mem::transmute(dwscope), ::core::mem::transmute(pvreserved1), ::core::mem::transmute(pvreserved2), ::core::mem::transmute(phcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardForgetCardTypeA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hcontext: usize, szcardname: Param1) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardForgetCardTypeA(hcontext: usize, szcardname: super::super::Foundation::PSTR) -> i32;
        }
        ::core::mem::transmute(SCardForgetCardTypeA(::core::mem::transmute(hcontext), szcardname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardForgetCardTypeW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hcontext: usize, szcardname: Param1) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardForgetCardTypeW(hcontext: usize, szcardname: super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(SCardForgetCardTypeW(::core::mem::transmute(hcontext), szcardname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardForgetReaderA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hcontext: usize, szreadername: Param1) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardForgetReaderA(hcontext: usize, szreadername: super::super::Foundation::PSTR) -> i32;
        }
        ::core::mem::transmute(SCardForgetReaderA(::core::mem::transmute(hcontext), szreadername.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardForgetReaderGroupA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hcontext: usize, szgroupname: Param1) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardForgetReaderGroupA(hcontext: usize, szgroupname: super::super::Foundation::PSTR) -> i32;
        }
        ::core::mem::transmute(SCardForgetReaderGroupA(::core::mem::transmute(hcontext), szgroupname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardForgetReaderGroupW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hcontext: usize, szgroupname: Param1) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardForgetReaderGroupW(hcontext: usize, szgroupname: super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(SCardForgetReaderGroupW(::core::mem::transmute(hcontext), szgroupname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardForgetReaderW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hcontext: usize, szreadername: Param1) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardForgetReaderW(hcontext: usize, szreadername: super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(SCardForgetReaderW(::core::mem::transmute(hcontext), szreadername.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
#[inline]
pub unsafe fn SCardFreeMemory(hcontext: usize, pvmem: *const ::core::ffi::c_void) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardFreeMemory(hcontext: usize, pvmem: *const ::core::ffi::c_void) -> i32;
        }
        ::core::mem::transmute(SCardFreeMemory(::core::mem::transmute(hcontext), ::core::mem::transmute(pvmem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
#[inline]
pub unsafe fn SCardGetAttrib(hcard: usize, dwattrid: u32, pbattr: *mut u8, pcbattrlen: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardGetAttrib(hcard: usize, dwattrid: u32, pbattr: *mut u8, pcbattrlen: *mut u32) -> i32;
        }
        ::core::mem::transmute(SCardGetAttrib(::core::mem::transmute(hcard), ::core::mem::transmute(dwattrid), ::core::mem::transmute(pbattr), ::core::mem::transmute(pcbattrlen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardGetCardTypeProviderNameA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hcontext: usize, szcardname: Param1, dwproviderid: u32, szprovider: super::super::Foundation::PSTR, pcchprovider: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardGetCardTypeProviderNameA(hcontext: usize, szcardname: super::super::Foundation::PSTR, dwproviderid: u32, szprovider: super::super::Foundation::PSTR, pcchprovider: *mut u32) -> i32;
        }
        ::core::mem::transmute(SCardGetCardTypeProviderNameA(::core::mem::transmute(hcontext), szcardname.into_param().abi(), ::core::mem::transmute(dwproviderid), ::core::mem::transmute(szprovider), ::core::mem::transmute(pcchprovider)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardGetCardTypeProviderNameW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hcontext: usize, szcardname: Param1, dwproviderid: u32, szprovider: super::super::Foundation::PWSTR, pcchprovider: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardGetCardTypeProviderNameW(hcontext: usize, szcardname: super::super::Foundation::PWSTR, dwproviderid: u32, szprovider: super::super::Foundation::PWSTR, pcchprovider: *mut u32) -> i32;
        }
        ::core::mem::transmute(SCardGetCardTypeProviderNameW(::core::mem::transmute(hcontext), szcardname.into_param().abi(), ::core::mem::transmute(dwproviderid), ::core::mem::transmute(szprovider), ::core::mem::transmute(pcchprovider)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardGetDeviceTypeIdA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hcontext: usize, szreadername: Param1, pdwdevicetypeid: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardGetDeviceTypeIdA(hcontext: usize, szreadername: super::super::Foundation::PSTR, pdwdevicetypeid: *mut u32) -> i32;
        }
        ::core::mem::transmute(SCardGetDeviceTypeIdA(::core::mem::transmute(hcontext), szreadername.into_param().abi(), ::core::mem::transmute(pdwdevicetypeid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardGetDeviceTypeIdW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hcontext: usize, szreadername: Param1, pdwdevicetypeid: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardGetDeviceTypeIdW(hcontext: usize, szreadername: super::super::Foundation::PWSTR, pdwdevicetypeid: *mut u32) -> i32;
        }
        ::core::mem::transmute(SCardGetDeviceTypeIdW(::core::mem::transmute(hcontext), szreadername.into_param().abi(), ::core::mem::transmute(pdwdevicetypeid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardGetProviderIdA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hcontext: usize, szcard: Param1, pguidproviderid: *mut ::windows::core::GUID) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardGetProviderIdA(hcontext: usize, szcard: super::super::Foundation::PSTR, pguidproviderid: *mut ::windows::core::GUID) -> i32;
        }
        ::core::mem::transmute(SCardGetProviderIdA(::core::mem::transmute(hcontext), szcard.into_param().abi(), ::core::mem::transmute(pguidproviderid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardGetProviderIdW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hcontext: usize, szcard: Param1, pguidproviderid: *mut ::windows::core::GUID) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardGetProviderIdW(hcontext: usize, szcard: super::super::Foundation::PWSTR, pguidproviderid: *mut ::windows::core::GUID) -> i32;
        }
        ::core::mem::transmute(SCardGetProviderIdW(::core::mem::transmute(hcontext), szcard.into_param().abi(), ::core::mem::transmute(pguidproviderid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardGetReaderDeviceInstanceIdA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hcontext: usize, szreadername: Param1, szdeviceinstanceid: super::super::Foundation::PSTR, pcchdeviceinstanceid: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardGetReaderDeviceInstanceIdA(hcontext: usize, szreadername: super::super::Foundation::PSTR, szdeviceinstanceid: super::super::Foundation::PSTR, pcchdeviceinstanceid: *mut u32) -> i32;
        }
        ::core::mem::transmute(SCardGetReaderDeviceInstanceIdA(::core::mem::transmute(hcontext), szreadername.into_param().abi(), ::core::mem::transmute(szdeviceinstanceid), ::core::mem::transmute(pcchdeviceinstanceid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardGetReaderDeviceInstanceIdW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hcontext: usize, szreadername: Param1, szdeviceinstanceid: super::super::Foundation::PWSTR, pcchdeviceinstanceid: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardGetReaderDeviceInstanceIdW(hcontext: usize, szreadername: super::super::Foundation::PWSTR, szdeviceinstanceid: super::super::Foundation::PWSTR, pcchdeviceinstanceid: *mut u32) -> i32;
        }
        ::core::mem::transmute(SCardGetReaderDeviceInstanceIdW(::core::mem::transmute(hcontext), szreadername.into_param().abi(), ::core::mem::transmute(szdeviceinstanceid), ::core::mem::transmute(pcchdeviceinstanceid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardGetReaderIconA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hcontext: usize, szreadername: Param1, pbicon: *mut u8, pcbicon: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardGetReaderIconA(hcontext: usize, szreadername: super::super::Foundation::PSTR, pbicon: *mut u8, pcbicon: *mut u32) -> i32;
        }
        ::core::mem::transmute(SCardGetReaderIconA(::core::mem::transmute(hcontext), szreadername.into_param().abi(), ::core::mem::transmute(pbicon), ::core::mem::transmute(pcbicon)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardGetReaderIconW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hcontext: usize, szreadername: Param1, pbicon: *mut u8, pcbicon: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardGetReaderIconW(hcontext: usize, szreadername: super::super::Foundation::PWSTR, pbicon: *mut u8, pcbicon: *mut u32) -> i32;
        }
        ::core::mem::transmute(SCardGetReaderIconW(::core::mem::transmute(hcontext), szreadername.into_param().abi(), ::core::mem::transmute(pbicon), ::core::mem::transmute(pcbicon)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardGetStatusChangeA(hcontext: usize, dwtimeout: u32, rgreaderstates: *mut SCARD_READERSTATEA, creaders: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardGetStatusChangeA(hcontext: usize, dwtimeout: u32, rgreaderstates: *mut SCARD_READERSTATEA, creaders: u32) -> i32;
        }
        ::core::mem::transmute(SCardGetStatusChangeA(::core::mem::transmute(hcontext), ::core::mem::transmute(dwtimeout), ::core::mem::transmute(rgreaderstates), ::core::mem::transmute(creaders)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardGetStatusChangeW(hcontext: usize, dwtimeout: u32, rgreaderstates: *mut SCARD_READERSTATEW, creaders: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardGetStatusChangeW(hcontext: usize, dwtimeout: u32, rgreaderstates: *mut SCARD_READERSTATEW, creaders: u32) -> i32;
        }
        ::core::mem::transmute(SCardGetStatusChangeW(::core::mem::transmute(hcontext), ::core::mem::transmute(dwtimeout), ::core::mem::transmute(rgreaderstates), ::core::mem::transmute(creaders)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
#[inline]
pub unsafe fn SCardGetTransmitCount(hcard: usize, pctransmitcount: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardGetTransmitCount(hcard: usize, pctransmitcount: *mut u32) -> i32;
        }
        ::core::mem::transmute(SCardGetTransmitCount(::core::mem::transmute(hcard), ::core::mem::transmute(pctransmitcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardIntroduceCardTypeA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hcontext: usize, szcardname: Param1, pguidprimaryprovider: *const ::windows::core::GUID, rgguidinterfaces: *const ::windows::core::GUID, dwinterfacecount: u32, pbatr: *const u8, pbatrmask: *const u8, cbatrlen: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardIntroduceCardTypeA(hcontext: usize, szcardname: super::super::Foundation::PSTR, pguidprimaryprovider: *const ::windows::core::GUID, rgguidinterfaces: *const ::windows::core::GUID, dwinterfacecount: u32, pbatr: *const u8, pbatrmask: *const u8, cbatrlen: u32) -> i32;
        }
        ::core::mem::transmute(SCardIntroduceCardTypeA(::core::mem::transmute(hcontext), szcardname.into_param().abi(), ::core::mem::transmute(pguidprimaryprovider), ::core::mem::transmute(rgguidinterfaces), ::core::mem::transmute(dwinterfacecount), ::core::mem::transmute(pbatr), ::core::mem::transmute(pbatrmask), ::core::mem::transmute(cbatrlen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardIntroduceCardTypeW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hcontext: usize, szcardname: Param1, pguidprimaryprovider: *const ::windows::core::GUID, rgguidinterfaces: *const ::windows::core::GUID, dwinterfacecount: u32, pbatr: *const u8, pbatrmask: *const u8, cbatrlen: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardIntroduceCardTypeW(hcontext: usize, szcardname: super::super::Foundation::PWSTR, pguidprimaryprovider: *const ::windows::core::GUID, rgguidinterfaces: *const ::windows::core::GUID, dwinterfacecount: u32, pbatr: *const u8, pbatrmask: *const u8, cbatrlen: u32) -> i32;
        }
        ::core::mem::transmute(SCardIntroduceCardTypeW(::core::mem::transmute(hcontext), szcardname.into_param().abi(), ::core::mem::transmute(pguidprimaryprovider), ::core::mem::transmute(rgguidinterfaces), ::core::mem::transmute(dwinterfacecount), ::core::mem::transmute(pbatr), ::core::mem::transmute(pbatrmask), ::core::mem::transmute(cbatrlen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardIntroduceReaderA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hcontext: usize, szreadername: Param1, szdevicename: Param2) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardIntroduceReaderA(hcontext: usize, szreadername: super::super::Foundation::PSTR, szdevicename: super::super::Foundation::PSTR) -> i32;
        }
        ::core::mem::transmute(SCardIntroduceReaderA(::core::mem::transmute(hcontext), szreadername.into_param().abi(), szdevicename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardIntroduceReaderGroupA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hcontext: usize, szgroupname: Param1) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardIntroduceReaderGroupA(hcontext: usize, szgroupname: super::super::Foundation::PSTR) -> i32;
        }
        ::core::mem::transmute(SCardIntroduceReaderGroupA(::core::mem::transmute(hcontext), szgroupname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardIntroduceReaderGroupW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hcontext: usize, szgroupname: Param1) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardIntroduceReaderGroupW(hcontext: usize, szgroupname: super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(SCardIntroduceReaderGroupW(::core::mem::transmute(hcontext), szgroupname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardIntroduceReaderW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hcontext: usize, szreadername: Param1, szdevicename: Param2) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardIntroduceReaderW(hcontext: usize, szreadername: super::super::Foundation::PWSTR, szdevicename: super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(SCardIntroduceReaderW(::core::mem::transmute(hcontext), szreadername.into_param().abi(), szdevicename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
#[inline]
pub unsafe fn SCardIsValidContext(hcontext: usize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardIsValidContext(hcontext: usize) -> i32;
        }
        ::core::mem::transmute(SCardIsValidContext(::core::mem::transmute(hcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardListCardsA(hcontext: usize, pbatr: *const u8, rgquidinterfaces: *const ::windows::core::GUID, cguidinterfacecount: u32, mszcards: super::super::Foundation::PSTR, pcchcards: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardListCardsA(hcontext: usize, pbatr: *const u8, rgquidinterfaces: *const ::windows::core::GUID, cguidinterfacecount: u32, mszcards: super::super::Foundation::PSTR, pcchcards: *mut u32) -> i32;
        }
        ::core::mem::transmute(SCardListCardsA(::core::mem::transmute(hcontext), ::core::mem::transmute(pbatr), ::core::mem::transmute(rgquidinterfaces), ::core::mem::transmute(cguidinterfacecount), ::core::mem::transmute(mszcards), ::core::mem::transmute(pcchcards)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardListCardsW(hcontext: usize, pbatr: *const u8, rgquidinterfaces: *const ::windows::core::GUID, cguidinterfacecount: u32, mszcards: super::super::Foundation::PWSTR, pcchcards: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardListCardsW(hcontext: usize, pbatr: *const u8, rgquidinterfaces: *const ::windows::core::GUID, cguidinterfacecount: u32, mszcards: super::super::Foundation::PWSTR, pcchcards: *mut u32) -> i32;
        }
        ::core::mem::transmute(SCardListCardsW(::core::mem::transmute(hcontext), ::core::mem::transmute(pbatr), ::core::mem::transmute(rgquidinterfaces), ::core::mem::transmute(cguidinterfacecount), ::core::mem::transmute(mszcards), ::core::mem::transmute(pcchcards)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardListInterfacesA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hcontext: usize, szcard: Param1, pguidinterfaces: *mut ::windows::core::GUID, pcguidinterfaces: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardListInterfacesA(hcontext: usize, szcard: super::super::Foundation::PSTR, pguidinterfaces: *mut ::windows::core::GUID, pcguidinterfaces: *mut u32) -> i32;
        }
        ::core::mem::transmute(SCardListInterfacesA(::core::mem::transmute(hcontext), szcard.into_param().abi(), ::core::mem::transmute(pguidinterfaces), ::core::mem::transmute(pcguidinterfaces)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardListInterfacesW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hcontext: usize, szcard: Param1, pguidinterfaces: *mut ::windows::core::GUID, pcguidinterfaces: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardListInterfacesW(hcontext: usize, szcard: super::super::Foundation::PWSTR, pguidinterfaces: *mut ::windows::core::GUID, pcguidinterfaces: *mut u32) -> i32;
        }
        ::core::mem::transmute(SCardListInterfacesW(::core::mem::transmute(hcontext), szcard.into_param().abi(), ::core::mem::transmute(pguidinterfaces), ::core::mem::transmute(pcguidinterfaces)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardListReaderGroupsA(hcontext: usize, mszgroups: super::super::Foundation::PSTR, pcchgroups: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardListReaderGroupsA(hcontext: usize, mszgroups: super::super::Foundation::PSTR, pcchgroups: *mut u32) -> i32;
        }
        ::core::mem::transmute(SCardListReaderGroupsA(::core::mem::transmute(hcontext), ::core::mem::transmute(mszgroups), ::core::mem::transmute(pcchgroups)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardListReaderGroupsW(hcontext: usize, mszgroups: super::super::Foundation::PWSTR, pcchgroups: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardListReaderGroupsW(hcontext: usize, mszgroups: super::super::Foundation::PWSTR, pcchgroups: *mut u32) -> i32;
        }
        ::core::mem::transmute(SCardListReaderGroupsW(::core::mem::transmute(hcontext), ::core::mem::transmute(mszgroups), ::core::mem::transmute(pcchgroups)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardListReadersA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hcontext: usize, mszgroups: Param1, mszreaders: super::super::Foundation::PSTR, pcchreaders: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardListReadersA(hcontext: usize, mszgroups: super::super::Foundation::PSTR, mszreaders: super::super::Foundation::PSTR, pcchreaders: *mut u32) -> i32;
        }
        ::core::mem::transmute(SCardListReadersA(::core::mem::transmute(hcontext), mszgroups.into_param().abi(), ::core::mem::transmute(mszreaders), ::core::mem::transmute(pcchreaders)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardListReadersW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hcontext: usize, mszgroups: Param1, mszreaders: super::super::Foundation::PWSTR, pcchreaders: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardListReadersW(hcontext: usize, mszgroups: super::super::Foundation::PWSTR, mszreaders: super::super::Foundation::PWSTR, pcchreaders: *mut u32) -> i32;
        }
        ::core::mem::transmute(SCardListReadersW(::core::mem::transmute(hcontext), mszgroups.into_param().abi(), ::core::mem::transmute(mszreaders), ::core::mem::transmute(pcchreaders)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardListReadersWithDeviceInstanceIdA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hcontext: usize, szdeviceinstanceid: Param1, mszreaders: super::super::Foundation::PSTR, pcchreaders: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardListReadersWithDeviceInstanceIdA(hcontext: usize, szdeviceinstanceid: super::super::Foundation::PSTR, mszreaders: super::super::Foundation::PSTR, pcchreaders: *mut u32) -> i32;
        }
        ::core::mem::transmute(SCardListReadersWithDeviceInstanceIdA(::core::mem::transmute(hcontext), szdeviceinstanceid.into_param().abi(), ::core::mem::transmute(mszreaders), ::core::mem::transmute(pcchreaders)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardListReadersWithDeviceInstanceIdW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hcontext: usize, szdeviceinstanceid: Param1, mszreaders: super::super::Foundation::PWSTR, pcchreaders: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardListReadersWithDeviceInstanceIdW(hcontext: usize, szdeviceinstanceid: super::super::Foundation::PWSTR, mszreaders: super::super::Foundation::PWSTR, pcchreaders: *mut u32) -> i32;
        }
        ::core::mem::transmute(SCardListReadersWithDeviceInstanceIdW(::core::mem::transmute(hcontext), szdeviceinstanceid.into_param().abi(), ::core::mem::transmute(mszreaders), ::core::mem::transmute(pcchreaders)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardLocateCardsA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hcontext: usize, mszcards: Param1, rgreaderstates: *mut SCARD_READERSTATEA, creaders: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardLocateCardsA(hcontext: usize, mszcards: super::super::Foundation::PSTR, rgreaderstates: *mut SCARD_READERSTATEA, creaders: u32) -> i32;
        }
        ::core::mem::transmute(SCardLocateCardsA(::core::mem::transmute(hcontext), mszcards.into_param().abi(), ::core::mem::transmute(rgreaderstates), ::core::mem::transmute(creaders)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardLocateCardsByATRA(hcontext: usize, rgatrmasks: *const SCARD_ATRMASK, catrs: u32, rgreaderstates: *mut SCARD_READERSTATEA, creaders: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardLocateCardsByATRA(hcontext: usize, rgatrmasks: *const SCARD_ATRMASK, catrs: u32, rgreaderstates: *mut SCARD_READERSTATEA, creaders: u32) -> i32;
        }
        ::core::mem::transmute(SCardLocateCardsByATRA(::core::mem::transmute(hcontext), ::core::mem::transmute(rgatrmasks), ::core::mem::transmute(catrs), ::core::mem::transmute(rgreaderstates), ::core::mem::transmute(creaders)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardLocateCardsByATRW(hcontext: usize, rgatrmasks: *const SCARD_ATRMASK, catrs: u32, rgreaderstates: *mut SCARD_READERSTATEW, creaders: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardLocateCardsByATRW(hcontext: usize, rgatrmasks: *const SCARD_ATRMASK, catrs: u32, rgreaderstates: *mut SCARD_READERSTATEW, creaders: u32) -> i32;
        }
        ::core::mem::transmute(SCardLocateCardsByATRW(::core::mem::transmute(hcontext), ::core::mem::transmute(rgatrmasks), ::core::mem::transmute(catrs), ::core::mem::transmute(rgreaderstates), ::core::mem::transmute(creaders)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardLocateCardsW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hcontext: usize, mszcards: Param1, rgreaderstates: *mut SCARD_READERSTATEW, creaders: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardLocateCardsW(hcontext: usize, mszcards: super::super::Foundation::PWSTR, rgreaderstates: *mut SCARD_READERSTATEW, creaders: u32) -> i32;
        }
        ::core::mem::transmute(SCardLocateCardsW(::core::mem::transmute(hcontext), mszcards.into_param().abi(), ::core::mem::transmute(rgreaderstates), ::core::mem::transmute(creaders)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardReadCacheA<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hcontext: usize, cardidentifier: *const ::windows::core::GUID, freshnesscounter: u32, lookupname: Param3, data: *mut u8, datalen: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardReadCacheA(hcontext: usize, cardidentifier: *const ::windows::core::GUID, freshnesscounter: u32, lookupname: super::super::Foundation::PSTR, data: *mut u8, datalen: *mut u32) -> i32;
        }
        ::core::mem::transmute(SCardReadCacheA(::core::mem::transmute(hcontext), ::core::mem::transmute(cardidentifier), ::core::mem::transmute(freshnesscounter), lookupname.into_param().abi(), ::core::mem::transmute(data), ::core::mem::transmute(datalen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardReadCacheW<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hcontext: usize, cardidentifier: *const ::windows::core::GUID, freshnesscounter: u32, lookupname: Param3, data: *mut u8, datalen: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardReadCacheW(hcontext: usize, cardidentifier: *const ::windows::core::GUID, freshnesscounter: u32, lookupname: super::super::Foundation::PWSTR, data: *mut u8, datalen: *mut u32) -> i32;
        }
        ::core::mem::transmute(SCardReadCacheW(::core::mem::transmute(hcontext), ::core::mem::transmute(cardidentifier), ::core::mem::transmute(freshnesscounter), lookupname.into_param().abi(), ::core::mem::transmute(data), ::core::mem::transmute(datalen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
#[inline]
pub unsafe fn SCardReconnect(hcard: usize, dwsharemode: u32, dwpreferredprotocols: u32, dwinitialization: u32, pdwactiveprotocol: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardReconnect(hcard: usize, dwsharemode: u32, dwpreferredprotocols: u32, dwinitialization: u32, pdwactiveprotocol: *mut u32) -> i32;
        }
        ::core::mem::transmute(SCardReconnect(::core::mem::transmute(hcard), ::core::mem::transmute(dwsharemode), ::core::mem::transmute(dwpreferredprotocols), ::core::mem::transmute(dwinitialization), ::core::mem::transmute(pdwactiveprotocol)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
#[inline]
pub unsafe fn SCardReleaseContext(hcontext: usize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardReleaseContext(hcontext: usize) -> i32;
        }
        ::core::mem::transmute(SCardReleaseContext(::core::mem::transmute(hcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
#[inline]
pub unsafe fn SCardReleaseStartedEvent() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardReleaseStartedEvent();
        }
        SCardReleaseStartedEvent()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardRemoveReaderFromGroupA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hcontext: usize, szreadername: Param1, szgroupname: Param2) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardRemoveReaderFromGroupA(hcontext: usize, szreadername: super::super::Foundation::PSTR, szgroupname: super::super::Foundation::PSTR) -> i32;
        }
        ::core::mem::transmute(SCardRemoveReaderFromGroupA(::core::mem::transmute(hcontext), szreadername.into_param().abi(), szgroupname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardRemoveReaderFromGroupW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hcontext: usize, szreadername: Param1, szgroupname: Param2) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardRemoveReaderFromGroupW(hcontext: usize, szreadername: super::super::Foundation::PWSTR, szgroupname: super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(SCardRemoveReaderFromGroupW(::core::mem::transmute(hcontext), szreadername.into_param().abi(), szgroupname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
#[inline]
pub unsafe fn SCardSetAttrib(hcard: usize, dwattrid: u32, pbattr: *const u8, cbattrlen: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardSetAttrib(hcard: usize, dwattrid: u32, pbattr: *const u8, cbattrlen: u32) -> i32;
        }
        ::core::mem::transmute(SCardSetAttrib(::core::mem::transmute(hcard), ::core::mem::transmute(dwattrid), ::core::mem::transmute(pbattr), ::core::mem::transmute(cbattrlen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardSetCardTypeProviderNameA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hcontext: usize, szcardname: Param1, dwproviderid: u32, szprovider: Param3) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardSetCardTypeProviderNameA(hcontext: usize, szcardname: super::super::Foundation::PSTR, dwproviderid: u32, szprovider: super::super::Foundation::PSTR) -> i32;
        }
        ::core::mem::transmute(SCardSetCardTypeProviderNameA(::core::mem::transmute(hcontext), szcardname.into_param().abi(), ::core::mem::transmute(dwproviderid), szprovider.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardSetCardTypeProviderNameW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hcontext: usize, szcardname: Param1, dwproviderid: u32, szprovider: Param3) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardSetCardTypeProviderNameW(hcontext: usize, szcardname: super::super::Foundation::PWSTR, dwproviderid: u32, szprovider: super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(SCardSetCardTypeProviderNameW(::core::mem::transmute(hcontext), szcardname.into_param().abi(), ::core::mem::transmute(dwproviderid), szprovider.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
#[inline]
pub unsafe fn SCardState(hcard: usize, pdwstate: *mut u32, pdwprotocol: *mut u32, pbatr: *mut u8, pcbatrlen: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardState(hcard: usize, pdwstate: *mut u32, pdwprotocol: *mut u32, pbatr: *mut u8, pcbatrlen: *mut u32) -> i32;
        }
        ::core::mem::transmute(SCardState(::core::mem::transmute(hcard), ::core::mem::transmute(pdwstate), ::core::mem::transmute(pdwprotocol), ::core::mem::transmute(pbatr), ::core::mem::transmute(pcbatrlen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardStatusA(hcard: usize, mszreadernames: super::super::Foundation::PSTR, pcchreaderlen: *mut u32, pdwstate: *mut u32, pdwprotocol: *mut u32, pbatr: *mut u8, pcbatrlen: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardStatusA(hcard: usize, mszreadernames: super::super::Foundation::PSTR, pcchreaderlen: *mut u32, pdwstate: *mut u32, pdwprotocol: *mut u32, pbatr: *mut u8, pcbatrlen: *mut u32) -> i32;
        }
        ::core::mem::transmute(SCardStatusA(::core::mem::transmute(hcard), ::core::mem::transmute(mszreadernames), ::core::mem::transmute(pcchreaderlen), ::core::mem::transmute(pdwstate), ::core::mem::transmute(pdwprotocol), ::core::mem::transmute(pbatr), ::core::mem::transmute(pcbatrlen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardStatusW(hcard: usize, mszreadernames: super::super::Foundation::PWSTR, pcchreaderlen: *mut u32, pdwstate: *mut u32, pdwprotocol: *mut u32, pbatr: *mut u8, pcbatrlen: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardStatusW(hcard: usize, mszreadernames: super::super::Foundation::PWSTR, pcchreaderlen: *mut u32, pdwstate: *mut u32, pdwprotocol: *mut u32, pbatr: *mut u8, pcbatrlen: *mut u32) -> i32;
        }
        ::core::mem::transmute(SCardStatusW(::core::mem::transmute(hcard), ::core::mem::transmute(mszreadernames), ::core::mem::transmute(pcchreaderlen), ::core::mem::transmute(pdwstate), ::core::mem::transmute(pdwprotocol), ::core::mem::transmute(pbatr), ::core::mem::transmute(pcbatrlen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
#[inline]
pub unsafe fn SCardTransmit(hcard: usize, piosendpci: *const SCARD_IO_REQUEST, pbsendbuffer: *const u8, cbsendlength: u32, piorecvpci: *mut SCARD_IO_REQUEST, pbrecvbuffer: *mut u8, pcbrecvlength: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardTransmit(hcard: usize, piosendpci: *const SCARD_IO_REQUEST, pbsendbuffer: *const u8, cbsendlength: u32, piorecvpci: *mut SCARD_IO_REQUEST, pbrecvbuffer: *mut u8, pcbrecvlength: *mut u32) -> i32;
        }
        ::core::mem::transmute(SCardTransmit(::core::mem::transmute(hcard), ::core::mem::transmute(piosendpci), ::core::mem::transmute(pbsendbuffer), ::core::mem::transmute(cbsendlength), ::core::mem::transmute(piorecvpci), ::core::mem::transmute(pbrecvbuffer), ::core::mem::transmute(pcbrecvlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation', 'Win32_UI_WindowsAndMessaging'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn SCardUIDlgSelectCardA(param0: *mut OPENCARDNAME_EXA) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardUIDlgSelectCardA(param0: *mut OPENCARDNAME_EXA) -> i32;
        }
        ::core::mem::transmute(SCardUIDlgSelectCardA(::core::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation', 'Win32_UI_WindowsAndMessaging'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn SCardUIDlgSelectCardW(param0: *mut OPENCARDNAME_EXW) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardUIDlgSelectCardW(param0: *mut OPENCARDNAME_EXW) -> i32;
        }
        ::core::mem::transmute(SCardUIDlgSelectCardW(::core::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardWriteCacheA<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hcontext: usize, cardidentifier: *const ::windows::core::GUID, freshnesscounter: u32, lookupname: Param3, data: *const u8, datalen: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardWriteCacheA(hcontext: usize, cardidentifier: *const ::windows::core::GUID, freshnesscounter: u32, lookupname: super::super::Foundation::PSTR, data: *const u8, datalen: u32) -> i32;
        }
        ::core::mem::transmute(SCardWriteCacheA(::core::mem::transmute(hcontext), ::core::mem::transmute(cardidentifier), ::core::mem::transmute(freshnesscounter), lookupname.into_param().abi(), ::core::mem::transmute(data), ::core::mem::transmute(datalen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardWriteCacheW<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hcontext: usize, cardidentifier: *const ::windows::core::GUID, freshnesscounter: u32, lookupname: Param3, data: *const u8, datalen: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardWriteCacheW(hcontext: usize, cardidentifier: *const ::windows::core::GUID, freshnesscounter: u32, lookupname: super::super::Foundation::PWSTR, data: *const u8, datalen: u32) -> i32;
        }
        ::core::mem::transmute(SCardWriteCacheW(::core::mem::transmute(hcontext), ::core::mem::transmute(cardidentifier), ::core::mem::transmute(freshnesscounter), lookupname.into_param().abi(), ::core::mem::transmute(data), ::core::mem::transmute(datalen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SECPKG_ALT_ATTR: u32 = 2147483648u32;
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub const SECPKG_ATTR_C_FULL_IDENT_TOKEN: u32 = 2147483781u32;
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_ACCESS_DENIED: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-1073741790i32);
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_ACCOUNT_DISABLED: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-1073741710i32);
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_ACCOUNT_EXPIRED: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-1073741421i32);
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_ACCOUNT_LOCKED_OUT: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-1073741260i32);
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_ACCOUNT_RESTRICTION: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-1073741714i32);
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_AUTHENTICATION_FIREWALL_FAILED: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-1073740781i32);
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_DOWNGRADE_DETECTED: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-1073740920i32);
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_LOGON_FAILURE: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-1073741715i32);
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_LOGON_TYPE_NOT_GRANTED: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-1073741477i32);
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_NO_SUCH_LOGON_SESSION: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-1073741729i32);
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_NO_SUCH_USER: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-1073741724i32);
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_PASSWORD_EXPIRED: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-1073741711i32);
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_PASSWORD_MUST_CHANGE: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-1073741276i32);
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_WRONG_PASSWORD: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-1073741718i32);
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub struct SecHandle {
    pub dwLower: usize,
    pub dwUpper: usize,
}
impl ::core::marker::Copy for SecHandle {}
impl ::core::clone::Clone for SecHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SecHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecHandle").field("dwLower", &self.dwLower).field("dwUpper", &self.dwUpper).finish()
    }
}
unsafe impl ::windows::core::Abi for SecHandle {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SecHandle {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SecHandle>()) == 0 }
    }
}
impl ::core::cmp::Eq for SecHandle {}
impl ::core::default::Default for SecHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_Credentials'*"]
pub struct SecPkgContext_ClientCreds {
    pub AuthBufferLen: u32,
    pub AuthBuffer: *mut u8,
}
impl ::core::marker::Copy for SecPkgContext_ClientCreds {}
impl ::core::clone::Clone for SecPkgContext_ClientCreds {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SecPkgContext_ClientCreds {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_ClientCreds").field("AuthBufferLen", &self.AuthBufferLen).field("AuthBuffer", &self.AuthBuffer).finish()
    }
}
unsafe impl ::windows::core::Abi for SecPkgContext_ClientCreds {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SecPkgContext_ClientCreds {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SecPkgContext_ClientCreds>()) == 0 }
    }
}
impl ::core::cmp::Eq for SecPkgContext_ClientCreds {}
impl ::core::default::Default for SecPkgContext_ClientCreds {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_Credentials', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USERNAME_TARGET_CREDENTIAL_INFO {
    pub UserName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USERNAME_TARGET_CREDENTIAL_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USERNAME_TARGET_CREDENTIAL_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for USERNAME_TARGET_CREDENTIAL_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USERNAME_TARGET_CREDENTIAL_INFO").field("UserName", &self.UserName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for USERNAME_TARGET_CREDENTIAL_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for USERNAME_TARGET_CREDENTIAL_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USERNAME_TARGET_CREDENTIAL_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for USERNAME_TARGET_CREDENTIAL_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USERNAME_TARGET_CREDENTIAL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
