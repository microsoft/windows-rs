#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct BINARY_BLOB_CREDENTIAL_INFO {
    pub cbBlob: u32,
    pub pbBlob: *mut u8,
}
impl BINARY_BLOB_CREDENTIAL_INFO {}
impl ::std::default::Default for BINARY_BLOB_CREDENTIAL_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for BINARY_BLOB_CREDENTIAL_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("BINARY_BLOB_CREDENTIAL_INFO")
            .field("cbBlob", &self.cbBlob)
            .field("pbBlob", &self.pbBlob)
            .finish()
    }
}
impl ::std::cmp::PartialEq for BINARY_BLOB_CREDENTIAL_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbBlob == other.cbBlob && self.pbBlob == other.pbBlob
    }
}
impl ::std::cmp::Eq for BINARY_BLOB_CREDENTIAL_INFO {}
unsafe impl ::windows::runtime::Abi for BINARY_BLOB_CREDENTIAL_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CERT_CREDENTIAL_INFO {
    pub cbSize: u32,
    pub rgbHashOfCert: [u8; 20],
}
impl CERT_CREDENTIAL_INFO {}
impl ::std::default::Default for CERT_CREDENTIAL_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CERT_CREDENTIAL_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CERT_CREDENTIAL_INFO")
            .field("cbSize", &self.cbSize)
            .field("rgbHashOfCert", &self.rgbHashOfCert)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CERT_CREDENTIAL_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.rgbHashOfCert == other.rgbHashOfCert
    }
}
impl ::std::cmp::Eq for CERT_CREDENTIAL_INFO {}
unsafe impl ::windows::runtime::Abi for CERT_CREDENTIAL_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
pub const CERT_HASH_LENGTH: u32 = 20u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl CREDENTIALA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CREDENTIALA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CREDENTIALA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CREDENTIALA")
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
impl ::std::cmp::PartialEq for CREDENTIALA {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags
            && self.Type == other.Type
            && self.TargetName == other.TargetName
            && self.Comment == other.Comment
            && self.LastWritten == other.LastWritten
            && self.CredentialBlobSize == other.CredentialBlobSize
            && self.CredentialBlob == other.CredentialBlob
            && self.Persist == other.Persist
            && self.AttributeCount == other.AttributeCount
            && self.Attributes == other.Attributes
            && self.TargetAlias == other.TargetAlias
            && self.UserName == other.UserName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CREDENTIALA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CREDENTIALA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl CREDENTIALW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CREDENTIALW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CREDENTIALW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CREDENTIALW")
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
impl ::std::cmp::PartialEq for CREDENTIALW {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags
            && self.Type == other.Type
            && self.TargetName == other.TargetName
            && self.Comment == other.Comment
            && self.LastWritten == other.LastWritten
            && self.CredentialBlobSize == other.CredentialBlobSize
            && self.CredentialBlob == other.CredentialBlob
            && self.Persist == other.Persist
            && self.AttributeCount == other.AttributeCount
            && self.Attributes == other.Attributes
            && self.TargetAlias == other.TargetAlias
            && self.UserName == other.UserName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CREDENTIALW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CREDENTIALW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CREDENTIAL_ATTRIBUTEA {
    pub Keyword: super::super::Foundation::PSTR,
    pub Flags: u32,
    pub ValueSize: u32,
    pub Value: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl CREDENTIAL_ATTRIBUTEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CREDENTIAL_ATTRIBUTEA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CREDENTIAL_ATTRIBUTEA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CREDENTIAL_ATTRIBUTEA")
            .field("Keyword", &self.Keyword)
            .field("Flags", &self.Flags)
            .field("ValueSize", &self.ValueSize)
            .field("Value", &self.Value)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CREDENTIAL_ATTRIBUTEA {
    fn eq(&self, other: &Self) -> bool {
        self.Keyword == other.Keyword
            && self.Flags == other.Flags
            && self.ValueSize == other.ValueSize
            && self.Value == other.Value
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CREDENTIAL_ATTRIBUTEA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CREDENTIAL_ATTRIBUTEA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CREDENTIAL_ATTRIBUTEW {
    pub Keyword: super::super::Foundation::PWSTR,
    pub Flags: u32,
    pub ValueSize: u32,
    pub Value: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl CREDENTIAL_ATTRIBUTEW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CREDENTIAL_ATTRIBUTEW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CREDENTIAL_ATTRIBUTEW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CREDENTIAL_ATTRIBUTEW")
            .field("Keyword", &self.Keyword)
            .field("Flags", &self.Flags)
            .field("ValueSize", &self.ValueSize)
            .field("Value", &self.Value)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CREDENTIAL_ATTRIBUTEW {
    fn eq(&self, other: &Self) -> bool {
        self.Keyword == other.Keyword
            && self.Flags == other.Flags
            && self.ValueSize == other.ValueSize
            && self.Value == other.Value
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CREDENTIAL_ATTRIBUTEW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CREDENTIAL_ATTRIBUTEW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl CREDENTIAL_TARGET_INFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CREDENTIAL_TARGET_INFORMATIONA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CREDENTIAL_TARGET_INFORMATIONA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CREDENTIAL_TARGET_INFORMATIONA")
            .field("TargetName", &self.TargetName)
            .field("NetbiosServerName", &self.NetbiosServerName)
            .field("DnsServerName", &self.DnsServerName)
            .field("NetbiosDomainName", &self.NetbiosDomainName)
            .field("DnsDomainName", &self.DnsDomainName)
            .field("DnsTreeName", &self.DnsTreeName)
            .field("PackageName", &self.PackageName)
            .field("Flags", &self.Flags)
            .field("CredTypeCount", &self.CredTypeCount)
            .field("CredTypes", &self.CredTypes)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CREDENTIAL_TARGET_INFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.TargetName == other.TargetName
            && self.NetbiosServerName == other.NetbiosServerName
            && self.DnsServerName == other.DnsServerName
            && self.NetbiosDomainName == other.NetbiosDomainName
            && self.DnsDomainName == other.DnsDomainName
            && self.DnsTreeName == other.DnsTreeName
            && self.PackageName == other.PackageName
            && self.Flags == other.Flags
            && self.CredTypeCount == other.CredTypeCount
            && self.CredTypes == other.CredTypes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CREDENTIAL_TARGET_INFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CREDENTIAL_TARGET_INFORMATIONA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl CREDENTIAL_TARGET_INFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CREDENTIAL_TARGET_INFORMATIONW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CREDENTIAL_TARGET_INFORMATIONW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CREDENTIAL_TARGET_INFORMATIONW")
            .field("TargetName", &self.TargetName)
            .field("NetbiosServerName", &self.NetbiosServerName)
            .field("DnsServerName", &self.DnsServerName)
            .field("NetbiosDomainName", &self.NetbiosDomainName)
            .field("DnsDomainName", &self.DnsDomainName)
            .field("DnsTreeName", &self.DnsTreeName)
            .field("PackageName", &self.PackageName)
            .field("Flags", &self.Flags)
            .field("CredTypeCount", &self.CredTypeCount)
            .field("CredTypes", &self.CredTypes)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CREDENTIAL_TARGET_INFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.TargetName == other.TargetName
            && self.NetbiosServerName == other.NetbiosServerName
            && self.DnsServerName == other.DnsServerName
            && self.NetbiosDomainName == other.NetbiosDomainName
            && self.DnsDomainName == other.DnsDomainName
            && self.DnsTreeName == other.DnsTreeName
            && self.PackageName == other.PackageName
            && self.Flags == other.Flags
            && self.CredTypeCount == other.CredTypeCount
            && self.CredTypes == other.CredTypes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CREDENTIAL_TARGET_INFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CREDENTIAL_TARGET_INFORMATIONW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CREDSPP_SUBMIT_TYPE(pub i32);
pub const CredsspPasswordCreds: CREDSPP_SUBMIT_TYPE = CREDSPP_SUBMIT_TYPE(2i32);
pub const CredsspSchannelCreds: CREDSPP_SUBMIT_TYPE = CREDSPP_SUBMIT_TYPE(4i32);
pub const CredsspCertificateCreds: CREDSPP_SUBMIT_TYPE = CREDSPP_SUBMIT_TYPE(13i32);
pub const CredsspSubmitBufferBoth: CREDSPP_SUBMIT_TYPE = CREDSPP_SUBMIT_TYPE(50i32);
pub const CredsspSubmitBufferBothOld: CREDSPP_SUBMIT_TYPE = CREDSPP_SUBMIT_TYPE(51i32);
pub const CredsspCredEx: CREDSPP_SUBMIT_TYPE = CREDSPP_SUBMIT_TYPE(100i32);
impl ::std::convert::From<i32> for CREDSPP_SUBMIT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CREDSPP_SUBMIT_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CREDSSP_CRED {
    pub Type: CREDSPP_SUBMIT_TYPE,
    pub pSchannelCred: *mut ::std::ffi::c_void,
    pub pSpnegoCred: *mut ::std::ffi::c_void,
}
impl CREDSSP_CRED {}
impl ::std::default::Default for CREDSSP_CRED {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CREDSSP_CRED {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CREDSSP_CRED")
            .field("Type", &self.Type)
            .field("pSchannelCred", &self.pSchannelCred)
            .field("pSpnegoCred", &self.pSpnegoCred)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CREDSSP_CRED {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type
            && self.pSchannelCred == other.pSchannelCred
            && self.pSpnegoCred == other.pSpnegoCred
    }
}
impl ::std::cmp::Eq for CREDSSP_CRED {}
unsafe impl ::windows::runtime::Abi for CREDSSP_CRED {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CREDSSP_CRED_EX {
    pub Type: CREDSPP_SUBMIT_TYPE,
    pub Version: u32,
    pub Flags: u32,
    pub Reserved: u32,
    pub Cred: CREDSSP_CRED,
}
impl CREDSSP_CRED_EX {}
impl ::std::default::Default for CREDSSP_CRED_EX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CREDSSP_CRED_EX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CREDSSP_CRED_EX")
            .field("Type", &self.Type)
            .field("Version", &self.Version)
            .field("Flags", &self.Flags)
            .field("Reserved", &self.Reserved)
            .field("Cred", &self.Cred)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CREDSSP_CRED_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type
            && self.Version == other.Version
            && self.Flags == other.Flags
            && self.Reserved == other.Reserved
            && self.Cred == other.Cred
    }
}
impl ::std::cmp::Eq for CREDSSP_CRED_EX {}
unsafe impl ::windows::runtime::Abi for CREDSSP_CRED_EX {
    type Abi = Self;
    type DefaultType = Self;
}
pub const CREDSSP_CRED_EX_VERSION: u32 = 0u32;
pub const CREDSSP_FLAG_REDIRECT: u32 = 1u32;
pub const CREDSSP_SERVER_AUTH_CERTIFICATE: u32 = 2u32;
pub const CREDSSP_SERVER_AUTH_LOOPBACK: u32 = 4u32;
pub const CREDSSP_SERVER_AUTH_NEGOTIATE: u32 = 1u32;
pub const CREDUIWIN_DOWNLEVEL_HELLO_AS_SMART_CARD: u32 = 2147483648u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CREDUIWIN_FLAGS(pub u32);
pub const CREDUIWIN_GENERIC: CREDUIWIN_FLAGS = CREDUIWIN_FLAGS(1u32);
pub const CREDUIWIN_CHECKBOX: CREDUIWIN_FLAGS = CREDUIWIN_FLAGS(2u32);
pub const CREDUIWIN_AUTHPACKAGE_ONLY: CREDUIWIN_FLAGS = CREDUIWIN_FLAGS(16u32);
pub const CREDUIWIN_IN_CRED_ONLY: CREDUIWIN_FLAGS = CREDUIWIN_FLAGS(32u32);
pub const CREDUIWIN_ENUMERATE_ADMINS: CREDUIWIN_FLAGS = CREDUIWIN_FLAGS(256u32);
pub const CREDUIWIN_ENUMERATE_CURRENT_USER: CREDUIWIN_FLAGS = CREDUIWIN_FLAGS(512u32);
pub const CREDUIWIN_SECURE_PROMPT: CREDUIWIN_FLAGS = CREDUIWIN_FLAGS(4096u32);
pub const CREDUIWIN_PREPROMPTING: CREDUIWIN_FLAGS = CREDUIWIN_FLAGS(8192u32);
pub const CREDUIWIN_PACK_32_WOW: CREDUIWIN_FLAGS = CREDUIWIN_FLAGS(268435456u32);
impl ::std::convert::From<u32> for CREDUIWIN_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CREDUIWIN_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CREDUIWIN_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CREDUIWIN_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CREDUIWIN_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CREDUIWIN_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CREDUIWIN_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CREDUIWIN_IGNORE_CLOUDAUTHORITY_NAME: u32 = 262144u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CREDUI_FLAGS(pub u32);
pub const CREDUI_FLAGS_ALWAYS_SHOW_UI: CREDUI_FLAGS = CREDUI_FLAGS(128u32);
pub const CREDUI_FLAGS_COMPLETE_USERNAME: CREDUI_FLAGS = CREDUI_FLAGS(2048u32);
pub const CREDUI_FLAGS_DO_NOT_PERSIST: CREDUI_FLAGS = CREDUI_FLAGS(2u32);
pub const CREDUI_FLAGS_EXCLUDE_CERTIFICATES: CREDUI_FLAGS = CREDUI_FLAGS(8u32);
pub const CREDUI_FLAGS_EXPECT_CONFIRMATION: CREDUI_FLAGS = CREDUI_FLAGS(131072u32);
pub const CREDUI_FLAGS_GENERIC_CREDENTIALS: CREDUI_FLAGS = CREDUI_FLAGS(262144u32);
pub const CREDUI_FLAGS_INCORRECT_PASSWORD: CREDUI_FLAGS = CREDUI_FLAGS(1u32);
pub const CREDUI_FLAGS_KEEP_USERNAME: CREDUI_FLAGS = CREDUI_FLAGS(1048576u32);
pub const CREDUI_FLAGS_PASSWORD_ONLY_OK: CREDUI_FLAGS = CREDUI_FLAGS(512u32);
pub const CREDUI_FLAGS_PERSIST: CREDUI_FLAGS = CREDUI_FLAGS(4096u32);
pub const CREDUI_FLAGS_REQUEST_ADMINISTRATOR: CREDUI_FLAGS = CREDUI_FLAGS(4u32);
pub const CREDUI_FLAGS_REQUIRE_CERTIFICATE: CREDUI_FLAGS = CREDUI_FLAGS(16u32);
pub const CREDUI_FLAGS_REQUIRE_SMARTCARD: CREDUI_FLAGS = CREDUI_FLAGS(256u32);
pub const CREDUI_FLAGS_SERVER_CREDENTIAL: CREDUI_FLAGS = CREDUI_FLAGS(16384u32);
pub const CREDUI_FLAGS_SHOW_SAVE_CHECK_BOX: CREDUI_FLAGS = CREDUI_FLAGS(64u32);
pub const CREDUI_FLAGS_USERNAME_TARGET_CREDENTIALS: CREDUI_FLAGS = CREDUI_FLAGS(524288u32);
pub const CREDUI_FLAGS_VALIDATE_USERNAME: CREDUI_FLAGS = CREDUI_FLAGS(1024u32);
impl ::std::convert::From<u32> for CREDUI_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CREDUI_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CREDUI_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CREDUI_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CREDUI_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CREDUI_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CREDUI_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct CREDUI_INFOA {
    pub cbSize: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub pszMessageText: super::super::Foundation::PSTR,
    pub pszCaptionText: super::super::Foundation::PSTR,
    pub hbmBanner: super::super::Graphics::Gdi::HBITMAP,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl CREDUI_INFOA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for CREDUI_INFOA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::fmt::Debug for CREDUI_INFOA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CREDUI_INFOA")
            .field("cbSize", &self.cbSize)
            .field("hwndParent", &self.hwndParent)
            .field("pszMessageText", &self.pszMessageText)
            .field("pszCaptionText", &self.pszCaptionText)
            .field("hbmBanner", &self.hbmBanner)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for CREDUI_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.hwndParent == other.hwndParent
            && self.pszMessageText == other.pszMessageText
            && self.pszCaptionText == other.pszCaptionText
            && self.hbmBanner == other.hbmBanner
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for CREDUI_INFOA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for CREDUI_INFOA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct CREDUI_INFOW {
    pub cbSize: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub pszMessageText: super::super::Foundation::PWSTR,
    pub pszCaptionText: super::super::Foundation::PWSTR,
    pub hbmBanner: super::super::Graphics::Gdi::HBITMAP,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl CREDUI_INFOW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for CREDUI_INFOW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::fmt::Debug for CREDUI_INFOW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CREDUI_INFOW")
            .field("cbSize", &self.cbSize)
            .field("hwndParent", &self.hwndParent)
            .field("pszMessageText", &self.pszMessageText)
            .field("pszCaptionText", &self.pszCaptionText)
            .field("hbmBanner", &self.hbmBanner)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for CREDUI_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.hwndParent == other.hwndParent
            && self.pszMessageText == other.pszMessageText
            && self.pszCaptionText == other.pszCaptionText
            && self.hbmBanner == other.hbmBanner
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for CREDUI_INFOW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for CREDUI_INFOW {
    type Abi = Self;
    type DefaultType = Self;
}
pub const CREDUI_MAX_CAPTION_LENGTH: u32 = 128u32;
pub const CREDUI_MAX_GENERIC_TARGET_LENGTH: u32 = 32767u32;
pub const CREDUI_MAX_MESSAGE_LENGTH: u32 = 1024u32;
pub const CRED_ALLOW_NAME_RESOLUTION: u32 = 1u32;
pub const CRED_CACHE_TARGET_INFORMATION: u32 = 1u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CRED_ENUMERATE_FLAGS(pub u32);
pub const CRED_ENUMERATE_ALL_CREDENTIALS: CRED_ENUMERATE_FLAGS = CRED_ENUMERATE_FLAGS(1u32);
impl ::std::convert::From<u32> for CRED_ENUMERATE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CRED_ENUMERATE_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CRED_ENUMERATE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CRED_ENUMERATE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CRED_ENUMERATE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CRED_ENUMERATE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CRED_ENUMERATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CRED_FLAGS(pub u32);
pub const CRED_FLAGS_PASSWORD_FOR_CERT: CRED_FLAGS = CRED_FLAGS(1u32);
pub const CRED_FLAGS_PROMPT_NOW: CRED_FLAGS = CRED_FLAGS(2u32);
pub const CRED_FLAGS_USERNAME_TARGET: CRED_FLAGS = CRED_FLAGS(4u32);
pub const CRED_FLAGS_OWF_CRED_BLOB: CRED_FLAGS = CRED_FLAGS(8u32);
pub const CRED_FLAGS_REQUIRE_CONFIRMATION: CRED_FLAGS = CRED_FLAGS(16u32);
pub const CRED_FLAGS_WILDCARD_MATCH: CRED_FLAGS = CRED_FLAGS(32u32);
pub const CRED_FLAGS_VSM_PROTECTED: CRED_FLAGS = CRED_FLAGS(64u32);
pub const CRED_FLAGS_NGC_CERT: CRED_FLAGS = CRED_FLAGS(128u32);
pub const CRED_FLAGS_VALID_FLAGS: CRED_FLAGS = CRED_FLAGS(61695u32);
pub const CRED_FLAGS_VALID_INPUT_FLAGS: CRED_FLAGS = CRED_FLAGS(61599u32);
impl ::std::convert::From<u32> for CRED_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CRED_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CRED_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CRED_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CRED_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CRED_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CRED_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CRED_LOGON_TYPES_MASK: u32 = 61440u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CRED_MARSHAL_TYPE(pub i32);
pub const CertCredential: CRED_MARSHAL_TYPE = CRED_MARSHAL_TYPE(1i32);
pub const UsernameTargetCredential: CRED_MARSHAL_TYPE = CRED_MARSHAL_TYPE(2i32);
pub const BinaryBlobCredential: CRED_MARSHAL_TYPE = CRED_MARSHAL_TYPE(3i32);
pub const UsernameForPackedCredentials: CRED_MARSHAL_TYPE = CRED_MARSHAL_TYPE(4i32);
pub const BinaryBlobForSystem: CRED_MARSHAL_TYPE = CRED_MARSHAL_TYPE(5i32);
impl ::std::convert::From<i32> for CRED_MARSHAL_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CRED_MARSHAL_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const CRED_MAX_ATTRIBUTES: u32 = 64u32;
pub const CRED_MAX_GENERIC_TARGET_NAME_LENGTH: u32 = 32767u32;
pub const CRED_MAX_STRING_LENGTH: u32 = 256u32;
pub const CRED_MAX_TARGETNAME_ATTRIBUTE_LENGTH: u32 = 256u32;
pub const CRED_MAX_TARGETNAME_NAMESPACE_LENGTH: u32 = 256u32;
pub const CRED_MAX_VALUE_SIZE: u32 = 256u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CRED_PACK_FLAGS(pub u32);
pub const CRED_PACK_PROTECTED_CREDENTIALS: CRED_PACK_FLAGS = CRED_PACK_FLAGS(1u32);
pub const CRED_PACK_WOW_BUFFER: CRED_PACK_FLAGS = CRED_PACK_FLAGS(2u32);
pub const CRED_PACK_GENERIC_CREDENTIALS: CRED_PACK_FLAGS = CRED_PACK_FLAGS(4u32);
pub const CRED_PACK_ID_PROVIDER_CREDENTIALS: CRED_PACK_FLAGS = CRED_PACK_FLAGS(8u32);
impl ::std::convert::From<u32> for CRED_PACK_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CRED_PACK_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CRED_PACK_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CRED_PACK_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CRED_PACK_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CRED_PACK_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CRED_PACK_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CRED_PERSIST(pub u32);
pub const CRED_PERSIST_NONE: CRED_PERSIST = CRED_PERSIST(0u32);
pub const CRED_PERSIST_SESSION: CRED_PERSIST = CRED_PERSIST(1u32);
pub const CRED_PERSIST_LOCAL_MACHINE: CRED_PERSIST = CRED_PERSIST(2u32);
pub const CRED_PERSIST_ENTERPRISE: CRED_PERSIST = CRED_PERSIST(3u32);
impl ::std::convert::From<u32> for CRED_PERSIST {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CRED_PERSIST {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CRED_PERSIST {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CRED_PERSIST {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CRED_PERSIST {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CRED_PERSIST {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CRED_PERSIST {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CRED_PRESERVE_CREDENTIAL_BLOB: u32 = 1u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CRED_PROTECTION_TYPE(pub i32);
pub const CredUnprotected: CRED_PROTECTION_TYPE = CRED_PROTECTION_TYPE(0i32);
pub const CredUserProtection: CRED_PROTECTION_TYPE = CRED_PROTECTION_TYPE(1i32);
pub const CredTrustedProtection: CRED_PROTECTION_TYPE = CRED_PROTECTION_TYPE(2i32);
pub const CredForSystemProtection: CRED_PROTECTION_TYPE = CRED_PROTECTION_TYPE(3i32);
impl ::std::convert::From<i32> for CRED_PROTECTION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CRED_PROTECTION_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const CRED_PROTECT_AS_SELF: u32 = 1u32;
pub const CRED_PROTECT_TO_SYSTEM: u32 = 2u32;
pub const CRED_TI_CREATE_EXPLICIT_CRED: u32 = 16u32;
pub const CRED_TI_DNSTREE_IS_DFS_SERVER: u32 = 64u32;
pub const CRED_TI_DOMAIN_FORMAT_UNKNOWN: u32 = 2u32;
pub const CRED_TI_ONLY_PASSWORD_REQUIRED: u32 = 4u32;
pub const CRED_TI_SERVER_FORMAT_UNKNOWN: u32 = 1u32;
pub const CRED_TI_USERNAME_TARGET: u32 = 8u32;
pub const CRED_TI_VALID_FLAGS: u32 = 61567u32;
pub const CRED_TI_WORKGROUP_MEMBER: u32 = 32u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CRED_TYPE(pub u32);
pub const CRED_TYPE_GENERIC: CRED_TYPE = CRED_TYPE(1u32);
pub const CRED_TYPE_DOMAIN_PASSWORD: CRED_TYPE = CRED_TYPE(2u32);
pub const CRED_TYPE_DOMAIN_CERTIFICATE: CRED_TYPE = CRED_TYPE(3u32);
pub const CRED_TYPE_DOMAIN_VISIBLE_PASSWORD: CRED_TYPE = CRED_TYPE(4u32);
pub const CRED_TYPE_GENERIC_CERTIFICATE: CRED_TYPE = CRED_TYPE(5u32);
pub const CRED_TYPE_DOMAIN_EXTENDED: CRED_TYPE = CRED_TYPE(6u32);
pub const CRED_TYPE_MAXIMUM: CRED_TYPE = CRED_TYPE(7u32);
pub const CRED_TYPE_MAXIMUM_EX: CRED_TYPE = CRED_TYPE(1007u32);
impl ::std::convert::From<u32> for CRED_TYPE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CRED_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CRED_TYPE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CRED_TYPE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CRED_TYPE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CRED_TYPE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CRED_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CRED_UNPROTECT_ALLOW_TO_SYSTEM: u32 = 2u32;
pub const CRED_UNPROTECT_AS_SELF: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredDeleteA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    targetname: Param0,
    r#type: u32,
    flags: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredDeleteA(
                targetname: super::super::Foundation::PSTR,
                r#type: u32,
                flags: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CredDeleteA(
            targetname.into_param().abi(),
            ::std::mem::transmute(r#type),
            ::std::mem::transmute(flags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredDeleteW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    targetname: Param0,
    r#type: u32,
    flags: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredDeleteW(
                targetname: super::super::Foundation::PWSTR,
                r#type: u32,
                flags: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CredDeleteW(
            targetname.into_param().abi(),
            ::std::mem::transmute(r#type),
            ::std::mem::transmute(flags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredEnumerateA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    filter: Param0,
    flags: CRED_ENUMERATE_FLAGS,
    count: *mut u32,
    credential: *mut *mut *mut CREDENTIALA,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredEnumerateA(
                filter: super::super::Foundation::PSTR,
                flags: CRED_ENUMERATE_FLAGS,
                count: *mut u32,
                credential: *mut *mut *mut CREDENTIALA,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CredEnumerateA(
            filter.into_param().abi(),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(count),
            ::std::mem::transmute(credential),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredEnumerateW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    filter: Param0,
    flags: CRED_ENUMERATE_FLAGS,
    count: *mut u32,
    credential: *mut *mut *mut CREDENTIALW,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredEnumerateW(
                filter: super::super::Foundation::PWSTR,
                flags: CRED_ENUMERATE_FLAGS,
                count: *mut u32,
                credential: *mut *mut *mut CREDENTIALW,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CredEnumerateW(
            filter.into_param().abi(),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(count),
            ::std::mem::transmute(credential),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredFindBestCredentialA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    targetname: Param0,
    r#type: u32,
    flags: u32,
    credential: *mut *mut CREDENTIALA,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredFindBestCredentialA(
                targetname: super::super::Foundation::PSTR,
                r#type: u32,
                flags: u32,
                credential: *mut *mut CREDENTIALA,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CredFindBestCredentialA(
            targetname.into_param().abi(),
            ::std::mem::transmute(r#type),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(credential),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredFindBestCredentialW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    targetname: Param0,
    r#type: u32,
    flags: u32,
    credential: *mut *mut CREDENTIALW,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredFindBestCredentialW(
                targetname: super::super::Foundation::PWSTR,
                r#type: u32,
                flags: u32,
                credential: *mut *mut CREDENTIALW,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CredFindBestCredentialW(
            targetname.into_param().abi(),
            ::std::mem::transmute(r#type),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(credential),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CredFree(buffer: *const ::std::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredFree(buffer: *const ::std::ffi::c_void);
        }
        ::std::mem::transmute(CredFree(::std::mem::transmute(buffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredGetSessionTypes(
    maximumpersistcount: u32,
    maximumpersist: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredGetSessionTypes(
                maximumpersistcount: u32,
                maximumpersist: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CredGetSessionTypes(
            ::std::mem::transmute(maximumpersistcount),
            ::std::mem::transmute(maximumpersist),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredGetTargetInfoA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    targetname: Param0,
    flags: u32,
    targetinfo: *mut *mut CREDENTIAL_TARGET_INFORMATIONA,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredGetTargetInfoA(
                targetname: super::super::Foundation::PSTR,
                flags: u32,
                targetinfo: *mut *mut CREDENTIAL_TARGET_INFORMATIONA,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CredGetTargetInfoA(
            targetname.into_param().abi(),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(targetinfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredGetTargetInfoW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    targetname: Param0,
    flags: u32,
    targetinfo: *mut *mut CREDENTIAL_TARGET_INFORMATIONW,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredGetTargetInfoW(
                targetname: super::super::Foundation::PWSTR,
                flags: u32,
                targetinfo: *mut *mut CREDENTIAL_TARGET_INFORMATIONW,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CredGetTargetInfoW(
            targetname.into_param().abi(),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(targetinfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredIsMarshaledCredentialA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    marshaledcredential: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredIsMarshaledCredentialA(
                marshaledcredential: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CredIsMarshaledCredentialA(
            marshaledcredential.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredIsMarshaledCredentialW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    marshaledcredential: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredIsMarshaledCredentialW(
                marshaledcredential: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CredIsMarshaledCredentialW(
            marshaledcredential.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredIsProtectedA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    pszprotectedcredentials: Param0,
    pprotectiontype: *mut CRED_PROTECTION_TYPE,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredIsProtectedA(
                pszprotectedcredentials: super::super::Foundation::PSTR,
                pprotectiontype: *mut CRED_PROTECTION_TYPE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CredIsProtectedA(
            pszprotectedcredentials.into_param().abi(),
            ::std::mem::transmute(pprotectiontype),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredIsProtectedW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    pszprotectedcredentials: Param0,
    pprotectiontype: *mut CRED_PROTECTION_TYPE,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredIsProtectedW(
                pszprotectedcredentials: super::super::Foundation::PWSTR,
                pprotectiontype: *mut CRED_PROTECTION_TYPE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CredIsProtectedW(
            pszprotectedcredentials.into_param().abi(),
            ::std::mem::transmute(pprotectiontype),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredMarshalCredentialA(
    credtype: CRED_MARSHAL_TYPE,
    credential: *const ::std::ffi::c_void,
    marshaledcredential: *mut super::super::Foundation::PSTR,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredMarshalCredentialA(
                credtype: CRED_MARSHAL_TYPE,
                credential: *const ::std::ffi::c_void,
                marshaledcredential: *mut super::super::Foundation::PSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CredMarshalCredentialA(
            ::std::mem::transmute(credtype),
            ::std::mem::transmute(credential),
            ::std::mem::transmute(marshaledcredential),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredMarshalCredentialW(
    credtype: CRED_MARSHAL_TYPE,
    credential: *const ::std::ffi::c_void,
    marshaledcredential: *mut super::super::Foundation::PWSTR,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredMarshalCredentialW(
                credtype: CRED_MARSHAL_TYPE,
                credential: *const ::std::ffi::c_void,
                marshaledcredential: *mut super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CredMarshalCredentialW(
            ::std::mem::transmute(credtype),
            ::std::mem::transmute(credential),
            ::std::mem::transmute(marshaledcredential),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredPackAuthenticationBufferA<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    dwflags: CRED_PACK_FLAGS,
    pszusername: Param1,
    pszpassword: Param2,
    ppackedcredentials: *mut u8,
    pcbpackedcredentials: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredPackAuthenticationBufferA(
                dwflags: CRED_PACK_FLAGS,
                pszusername: super::super::Foundation::PSTR,
                pszpassword: super::super::Foundation::PSTR,
                ppackedcredentials: *mut u8,
                pcbpackedcredentials: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CredPackAuthenticationBufferA(
            ::std::mem::transmute(dwflags),
            pszusername.into_param().abi(),
            pszpassword.into_param().abi(),
            ::std::mem::transmute(ppackedcredentials),
            ::std::mem::transmute(pcbpackedcredentials),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredPackAuthenticationBufferW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    dwflags: CRED_PACK_FLAGS,
    pszusername: Param1,
    pszpassword: Param2,
    ppackedcredentials: *mut u8,
    pcbpackedcredentials: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredPackAuthenticationBufferW(
                dwflags: CRED_PACK_FLAGS,
                pszusername: super::super::Foundation::PWSTR,
                pszpassword: super::super::Foundation::PWSTR,
                ppackedcredentials: *mut u8,
                pcbpackedcredentials: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CredPackAuthenticationBufferW(
            ::std::mem::transmute(dwflags),
            pszusername.into_param().abi(),
            pszpassword.into_param().abi(),
            ::std::mem::transmute(ppackedcredentials),
            ::std::mem::transmute(pcbpackedcredentials),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredProtectA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    fasself: Param0,
    pszcredentials: Param1,
    cchcredentials: u32,
    pszprotectedcredentials: super::super::Foundation::PSTR,
    pcchmaxchars: *mut u32,
    protectiontype: *mut CRED_PROTECTION_TYPE,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredProtectA(
                fasself: super::super::Foundation::BOOL,
                pszcredentials: super::super::Foundation::PSTR,
                cchcredentials: u32,
                pszprotectedcredentials: super::super::Foundation::PSTR,
                pcchmaxchars: *mut u32,
                protectiontype: *mut CRED_PROTECTION_TYPE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CredProtectA(
            fasself.into_param().abi(),
            pszcredentials.into_param().abi(),
            ::std::mem::transmute(cchcredentials),
            ::std::mem::transmute(pszprotectedcredentials),
            ::std::mem::transmute(pcchmaxchars),
            ::std::mem::transmute(protectiontype),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredProtectW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    fasself: Param0,
    pszcredentials: Param1,
    cchcredentials: u32,
    pszprotectedcredentials: super::super::Foundation::PWSTR,
    pcchmaxchars: *mut u32,
    protectiontype: *mut CRED_PROTECTION_TYPE,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredProtectW(
                fasself: super::super::Foundation::BOOL,
                pszcredentials: super::super::Foundation::PWSTR,
                cchcredentials: u32,
                pszprotectedcredentials: super::super::Foundation::PWSTR,
                pcchmaxchars: *mut u32,
                protectiontype: *mut CRED_PROTECTION_TYPE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CredProtectW(
            fasself.into_param().abi(),
            pszcredentials.into_param().abi(),
            ::std::mem::transmute(cchcredentials),
            ::std::mem::transmute(pszprotectedcredentials),
            ::std::mem::transmute(pcchmaxchars),
            ::std::mem::transmute(protectiontype),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredReadA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    targetname: Param0,
    r#type: u32,
    flags: u32,
    credential: *mut *mut CREDENTIALA,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredReadA(
                targetname: super::super::Foundation::PSTR,
                r#type: u32,
                flags: u32,
                credential: *mut *mut CREDENTIALA,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CredReadA(
            targetname.into_param().abi(),
            ::std::mem::transmute(r#type),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(credential),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredReadDomainCredentialsA(
    targetinfo: *const CREDENTIAL_TARGET_INFORMATIONA,
    flags: u32,
    count: *mut u32,
    credential: *mut *mut *mut CREDENTIALA,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredReadDomainCredentialsA(
                targetinfo: *const CREDENTIAL_TARGET_INFORMATIONA,
                flags: u32,
                count: *mut u32,
                credential: *mut *mut *mut CREDENTIALA,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CredReadDomainCredentialsA(
            ::std::mem::transmute(targetinfo),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(count),
            ::std::mem::transmute(credential),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredReadDomainCredentialsW(
    targetinfo: *const CREDENTIAL_TARGET_INFORMATIONW,
    flags: u32,
    count: *mut u32,
    credential: *mut *mut *mut CREDENTIALW,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredReadDomainCredentialsW(
                targetinfo: *const CREDENTIAL_TARGET_INFORMATIONW,
                flags: u32,
                count: *mut u32,
                credential: *mut *mut *mut CREDENTIALW,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CredReadDomainCredentialsW(
            ::std::mem::transmute(targetinfo),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(count),
            ::std::mem::transmute(credential),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredReadW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    targetname: Param0,
    r#type: u32,
    flags: u32,
    credential: *mut *mut CREDENTIALW,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredReadW(
                targetname: super::super::Foundation::PWSTR,
                r#type: u32,
                flags: u32,
                credential: *mut *mut CREDENTIALW,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CredReadW(
            targetname.into_param().abi(),
            ::std::mem::transmute(r#type),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(credential),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredRenameA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    oldtargetname: Param0,
    newtargetname: Param1,
    r#type: u32,
    flags: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredRenameA(
                oldtargetname: super::super::Foundation::PSTR,
                newtargetname: super::super::Foundation::PSTR,
                r#type: u32,
                flags: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CredRenameA(
            oldtargetname.into_param().abi(),
            newtargetname.into_param().abi(),
            ::std::mem::transmute(r#type),
            ::std::mem::transmute(flags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredRenameW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    oldtargetname: Param0,
    newtargetname: Param1,
    r#type: u32,
    flags: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredRenameW(
                oldtargetname: super::super::Foundation::PWSTR,
                newtargetname: super::super::Foundation::PWSTR,
                r#type: u32,
                flags: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CredRenameW(
            oldtargetname.into_param().abi(),
            newtargetname.into_param().abi(),
            ::std::mem::transmute(r#type),
            ::std::mem::transmute(flags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredUICmdLinePromptForCredentialsA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    psztargetname: Param0,
    pcontext: *mut SecHandle,
    dwautherror: u32,
    username: Param3,
    uluserbuffersize: u32,
    pszpassword: Param5,
    ulpasswordbuffersize: u32,
    pfsave: *mut super::super::Foundation::BOOL,
    dwflags: CREDUI_FLAGS,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredUICmdLinePromptForCredentialsA(
                psztargetname: super::super::Foundation::PSTR,
                pcontext: *mut SecHandle,
                dwautherror: u32,
                username: super::super::Foundation::PSTR,
                uluserbuffersize: u32,
                pszpassword: super::super::Foundation::PSTR,
                ulpasswordbuffersize: u32,
                pfsave: *mut super::super::Foundation::BOOL,
                dwflags: CREDUI_FLAGS,
            ) -> u32;
        }
        ::std::mem::transmute(CredUICmdLinePromptForCredentialsA(
            psztargetname.into_param().abi(),
            ::std::mem::transmute(pcontext),
            ::std::mem::transmute(dwautherror),
            username.into_param().abi(),
            ::std::mem::transmute(uluserbuffersize),
            pszpassword.into_param().abi(),
            ::std::mem::transmute(ulpasswordbuffersize),
            ::std::mem::transmute(pfsave),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredUICmdLinePromptForCredentialsW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    psztargetname: Param0,
    pcontext: *mut SecHandle,
    dwautherror: u32,
    username: Param3,
    uluserbuffersize: u32,
    pszpassword: Param5,
    ulpasswordbuffersize: u32,
    pfsave: *mut super::super::Foundation::BOOL,
    dwflags: CREDUI_FLAGS,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredUICmdLinePromptForCredentialsW(
                psztargetname: super::super::Foundation::PWSTR,
                pcontext: *mut SecHandle,
                dwautherror: u32,
                username: super::super::Foundation::PWSTR,
                uluserbuffersize: u32,
                pszpassword: super::super::Foundation::PWSTR,
                ulpasswordbuffersize: u32,
                pfsave: *mut super::super::Foundation::BOOL,
                dwflags: CREDUI_FLAGS,
            ) -> u32;
        }
        ::std::mem::transmute(CredUICmdLinePromptForCredentialsW(
            psztargetname.into_param().abi(),
            ::std::mem::transmute(pcontext),
            ::std::mem::transmute(dwautherror),
            username.into_param().abi(),
            ::std::mem::transmute(uluserbuffersize),
            pszpassword.into_param().abi(),
            ::std::mem::transmute(ulpasswordbuffersize),
            ::std::mem::transmute(pfsave),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredUIConfirmCredentialsA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    psztargetname: Param0,
    bconfirm: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredUIConfirmCredentialsA(
                psztargetname: super::super::Foundation::PSTR,
                bconfirm: super::super::Foundation::BOOL,
            ) -> u32;
        }
        ::std::mem::transmute(CredUIConfirmCredentialsA(
            psztargetname.into_param().abi(),
            bconfirm.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredUIConfirmCredentialsW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    psztargetname: Param0,
    bconfirm: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredUIConfirmCredentialsW(
                psztargetname: super::super::Foundation::PWSTR,
                bconfirm: super::super::Foundation::BOOL,
            ) -> u32;
        }
        ::std::mem::transmute(CredUIConfirmCredentialsW(
            psztargetname.into_param().abi(),
            bconfirm.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredUIParseUserNameA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    username: Param0,
    user: super::super::Foundation::PSTR,
    userbuffersize: u32,
    domain: super::super::Foundation::PSTR,
    domainbuffersize: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredUIParseUserNameA(
                username: super::super::Foundation::PSTR,
                user: super::super::Foundation::PSTR,
                userbuffersize: u32,
                domain: super::super::Foundation::PSTR,
                domainbuffersize: u32,
            ) -> u32;
        }
        ::std::mem::transmute(CredUIParseUserNameA(
            username.into_param().abi(),
            ::std::mem::transmute(user),
            ::std::mem::transmute(userbuffersize),
            ::std::mem::transmute(domain),
            ::std::mem::transmute(domainbuffersize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredUIParseUserNameW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    username: Param0,
    user: super::super::Foundation::PWSTR,
    userbuffersize: u32,
    domain: super::super::Foundation::PWSTR,
    domainbuffersize: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredUIParseUserNameW(
                username: super::super::Foundation::PWSTR,
                user: super::super::Foundation::PWSTR,
                userbuffersize: u32,
                domain: super::super::Foundation::PWSTR,
                domainbuffersize: u32,
            ) -> u32;
        }
        ::std::mem::transmute(CredUIParseUserNameW(
            username.into_param().abi(),
            ::std::mem::transmute(user),
            ::std::mem::transmute(userbuffersize),
            ::std::mem::transmute(domain),
            ::std::mem::transmute(domainbuffersize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn CredUIPromptForCredentialsA<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    puiinfo: *const CREDUI_INFOA,
    psztargetname: Param1,
    pcontext: *mut SecHandle,
    dwautherror: u32,
    pszusername: Param4,
    ulusernamebuffersize: u32,
    pszpassword: Param6,
    ulpasswordbuffersize: u32,
    save: *mut super::super::Foundation::BOOL,
    dwflags: CREDUI_FLAGS,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredUIPromptForCredentialsA(
                puiinfo: *const CREDUI_INFOA,
                psztargetname: super::super::Foundation::PSTR,
                pcontext: *mut SecHandle,
                dwautherror: u32,
                pszusername: super::super::Foundation::PSTR,
                ulusernamebuffersize: u32,
                pszpassword: super::super::Foundation::PSTR,
                ulpasswordbuffersize: u32,
                save: *mut super::super::Foundation::BOOL,
                dwflags: CREDUI_FLAGS,
            ) -> u32;
        }
        ::std::mem::transmute(CredUIPromptForCredentialsA(
            ::std::mem::transmute(puiinfo),
            psztargetname.into_param().abi(),
            ::std::mem::transmute(pcontext),
            ::std::mem::transmute(dwautherror),
            pszusername.into_param().abi(),
            ::std::mem::transmute(ulusernamebuffersize),
            pszpassword.into_param().abi(),
            ::std::mem::transmute(ulpasswordbuffersize),
            ::std::mem::transmute(save),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn CredUIPromptForCredentialsW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    puiinfo: *const CREDUI_INFOW,
    psztargetname: Param1,
    pcontext: *mut SecHandle,
    dwautherror: u32,
    pszusername: Param4,
    ulusernamebuffersize: u32,
    pszpassword: Param6,
    ulpasswordbuffersize: u32,
    save: *mut super::super::Foundation::BOOL,
    dwflags: CREDUI_FLAGS,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredUIPromptForCredentialsW(
                puiinfo: *const CREDUI_INFOW,
                psztargetname: super::super::Foundation::PWSTR,
                pcontext: *mut SecHandle,
                dwautherror: u32,
                pszusername: super::super::Foundation::PWSTR,
                ulusernamebuffersize: u32,
                pszpassword: super::super::Foundation::PWSTR,
                ulpasswordbuffersize: u32,
                save: *mut super::super::Foundation::BOOL,
                dwflags: CREDUI_FLAGS,
            ) -> u32;
        }
        ::std::mem::transmute(CredUIPromptForCredentialsW(
            ::std::mem::transmute(puiinfo),
            psztargetname.into_param().abi(),
            ::std::mem::transmute(pcontext),
            ::std::mem::transmute(dwautherror),
            pszusername.into_param().abi(),
            ::std::mem::transmute(ulusernamebuffersize),
            pszpassword.into_param().abi(),
            ::std::mem::transmute(ulpasswordbuffersize),
            ::std::mem::transmute(save),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn CredUIPromptForWindowsCredentialsA(
    puiinfo: *const CREDUI_INFOA,
    dwautherror: u32,
    pulauthpackage: *mut u32,
    pvinauthbuffer: *const ::std::ffi::c_void,
    ulinauthbuffersize: u32,
    ppvoutauthbuffer: *mut *mut ::std::ffi::c_void,
    puloutauthbuffersize: *mut u32,
    pfsave: *mut super::super::Foundation::BOOL,
    dwflags: CREDUIWIN_FLAGS,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredUIPromptForWindowsCredentialsA(
                puiinfo: *const CREDUI_INFOA,
                dwautherror: u32,
                pulauthpackage: *mut u32,
                pvinauthbuffer: *const ::std::ffi::c_void,
                ulinauthbuffersize: u32,
                ppvoutauthbuffer: *mut *mut ::std::ffi::c_void,
                puloutauthbuffersize: *mut u32,
                pfsave: *mut super::super::Foundation::BOOL,
                dwflags: CREDUIWIN_FLAGS,
            ) -> u32;
        }
        ::std::mem::transmute(CredUIPromptForWindowsCredentialsA(
            ::std::mem::transmute(puiinfo),
            ::std::mem::transmute(dwautherror),
            ::std::mem::transmute(pulauthpackage),
            ::std::mem::transmute(pvinauthbuffer),
            ::std::mem::transmute(ulinauthbuffersize),
            ::std::mem::transmute(ppvoutauthbuffer),
            ::std::mem::transmute(puloutauthbuffersize),
            ::std::mem::transmute(pfsave),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn CredUIPromptForWindowsCredentialsW(
    puiinfo: *const CREDUI_INFOW,
    dwautherror: u32,
    pulauthpackage: *mut u32,
    pvinauthbuffer: *const ::std::ffi::c_void,
    ulinauthbuffersize: u32,
    ppvoutauthbuffer: *mut *mut ::std::ffi::c_void,
    puloutauthbuffersize: *mut u32,
    pfsave: *mut super::super::Foundation::BOOL,
    dwflags: CREDUIWIN_FLAGS,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredUIPromptForWindowsCredentialsW(
                puiinfo: *const CREDUI_INFOW,
                dwautherror: u32,
                pulauthpackage: *mut u32,
                pvinauthbuffer: *const ::std::ffi::c_void,
                ulinauthbuffersize: u32,
                ppvoutauthbuffer: *mut *mut ::std::ffi::c_void,
                puloutauthbuffersize: *mut u32,
                pfsave: *mut super::super::Foundation::BOOL,
                dwflags: CREDUIWIN_FLAGS,
            ) -> u32;
        }
        ::std::mem::transmute(CredUIPromptForWindowsCredentialsW(
            ::std::mem::transmute(puiinfo),
            ::std::mem::transmute(dwautherror),
            ::std::mem::transmute(pulauthpackage),
            ::std::mem::transmute(pvinauthbuffer),
            ::std::mem::transmute(ulinauthbuffersize),
            ::std::mem::transmute(ppvoutauthbuffer),
            ::std::mem::transmute(puloutauthbuffersize),
            ::std::mem::transmute(pfsave),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredUIReadSSOCredW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    pszrealm: Param0,
    ppszusername: *mut super::super::Foundation::PWSTR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredUIReadSSOCredW(
                pszrealm: super::super::Foundation::PWSTR,
                ppszusername: *mut super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(CredUIReadSSOCredW(
            pszrealm.into_param().abi(),
            ::std::mem::transmute(ppszusername),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredUIStoreSSOCredW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    pszrealm: Param0,
    pszusername: Param1,
    pszpassword: Param2,
    bpersist: Param3,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredUIStoreSSOCredW(
                pszrealm: super::super::Foundation::PWSTR,
                pszusername: super::super::Foundation::PWSTR,
                pszpassword: super::super::Foundation::PWSTR,
                bpersist: super::super::Foundation::BOOL,
            ) -> u32;
        }
        ::std::mem::transmute(CredUIStoreSSOCredW(
            pszrealm.into_param().abi(),
            pszusername.into_param().abi(),
            pszpassword.into_param().abi(),
            bpersist.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredUnPackAuthenticationBufferA(
    dwflags: CRED_PACK_FLAGS,
    pauthbuffer: *const ::std::ffi::c_void,
    cbauthbuffer: u32,
    pszusername: super::super::Foundation::PSTR,
    pcchlmaxusername: *mut u32,
    pszdomainname: super::super::Foundation::PSTR,
    pcchmaxdomainname: *mut u32,
    pszpassword: super::super::Foundation::PSTR,
    pcchmaxpassword: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredUnPackAuthenticationBufferA(
                dwflags: CRED_PACK_FLAGS,
                pauthbuffer: *const ::std::ffi::c_void,
                cbauthbuffer: u32,
                pszusername: super::super::Foundation::PSTR,
                pcchlmaxusername: *mut u32,
                pszdomainname: super::super::Foundation::PSTR,
                pcchmaxdomainname: *mut u32,
                pszpassword: super::super::Foundation::PSTR,
                pcchmaxpassword: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CredUnPackAuthenticationBufferA(
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(pauthbuffer),
            ::std::mem::transmute(cbauthbuffer),
            ::std::mem::transmute(pszusername),
            ::std::mem::transmute(pcchlmaxusername),
            ::std::mem::transmute(pszdomainname),
            ::std::mem::transmute(pcchmaxdomainname),
            ::std::mem::transmute(pszpassword),
            ::std::mem::transmute(pcchmaxpassword),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredUnPackAuthenticationBufferW(
    dwflags: CRED_PACK_FLAGS,
    pauthbuffer: *const ::std::ffi::c_void,
    cbauthbuffer: u32,
    pszusername: super::super::Foundation::PWSTR,
    pcchmaxusername: *mut u32,
    pszdomainname: super::super::Foundation::PWSTR,
    pcchmaxdomainname: *mut u32,
    pszpassword: super::super::Foundation::PWSTR,
    pcchmaxpassword: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredUnPackAuthenticationBufferW(
                dwflags: CRED_PACK_FLAGS,
                pauthbuffer: *const ::std::ffi::c_void,
                cbauthbuffer: u32,
                pszusername: super::super::Foundation::PWSTR,
                pcchmaxusername: *mut u32,
                pszdomainname: super::super::Foundation::PWSTR,
                pcchmaxdomainname: *mut u32,
                pszpassword: super::super::Foundation::PWSTR,
                pcchmaxpassword: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CredUnPackAuthenticationBufferW(
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(pauthbuffer),
            ::std::mem::transmute(cbauthbuffer),
            ::std::mem::transmute(pszusername),
            ::std::mem::transmute(pcchmaxusername),
            ::std::mem::transmute(pszdomainname),
            ::std::mem::transmute(pcchmaxdomainname),
            ::std::mem::transmute(pszpassword),
            ::std::mem::transmute(pcchmaxpassword),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredUnmarshalCredentialA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    marshaledcredential: Param0,
    credtype: *mut CRED_MARSHAL_TYPE,
    credential: *mut *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredUnmarshalCredentialA(
                marshaledcredential: super::super::Foundation::PSTR,
                credtype: *mut CRED_MARSHAL_TYPE,
                credential: *mut *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CredUnmarshalCredentialA(
            marshaledcredential.into_param().abi(),
            ::std::mem::transmute(credtype),
            ::std::mem::transmute(credential),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredUnmarshalCredentialW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    marshaledcredential: Param0,
    credtype: *mut CRED_MARSHAL_TYPE,
    credential: *mut *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredUnmarshalCredentialW(
                marshaledcredential: super::super::Foundation::PWSTR,
                credtype: *mut CRED_MARSHAL_TYPE,
                credential: *mut *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CredUnmarshalCredentialW(
            marshaledcredential.into_param().abi(),
            ::std::mem::transmute(credtype),
            ::std::mem::transmute(credential),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredUnprotectA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    fasself: Param0,
    pszprotectedcredentials: Param1,
    cchprotectedcredentials: u32,
    pszcredentials: super::super::Foundation::PSTR,
    pcchmaxchars: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredUnprotectA(
                fasself: super::super::Foundation::BOOL,
                pszprotectedcredentials: super::super::Foundation::PSTR,
                cchprotectedcredentials: u32,
                pszcredentials: super::super::Foundation::PSTR,
                pcchmaxchars: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CredUnprotectA(
            fasself.into_param().abi(),
            pszprotectedcredentials.into_param().abi(),
            ::std::mem::transmute(cchprotectedcredentials),
            ::std::mem::transmute(pszcredentials),
            ::std::mem::transmute(pcchmaxchars),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredUnprotectW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    fasself: Param0,
    pszprotectedcredentials: Param1,
    cchprotectedcredentials: u32,
    pszcredentials: super::super::Foundation::PWSTR,
    pcchmaxchars: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredUnprotectW(
                fasself: super::super::Foundation::BOOL,
                pszprotectedcredentials: super::super::Foundation::PWSTR,
                cchprotectedcredentials: u32,
                pszcredentials: super::super::Foundation::PWSTR,
                pcchmaxchars: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CredUnprotectW(
            fasself.into_param().abi(),
            pszprotectedcredentials.into_param().abi(),
            ::std::mem::transmute(cchprotectedcredentials),
            ::std::mem::transmute(pszcredentials),
            ::std::mem::transmute(pcchmaxchars),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredWriteA(
    credential: *const CREDENTIALA,
    flags: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredWriteA(
                credential: *const CREDENTIALA,
                flags: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CredWriteA(
            ::std::mem::transmute(credential),
            ::std::mem::transmute(flags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredWriteDomainCredentialsA(
    targetinfo: *const CREDENTIAL_TARGET_INFORMATIONA,
    credential: *const CREDENTIALA,
    flags: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredWriteDomainCredentialsA(
                targetinfo: *const CREDENTIAL_TARGET_INFORMATIONA,
                credential: *const CREDENTIALA,
                flags: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CredWriteDomainCredentialsA(
            ::std::mem::transmute(targetinfo),
            ::std::mem::transmute(credential),
            ::std::mem::transmute(flags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredWriteDomainCredentialsW(
    targetinfo: *const CREDENTIAL_TARGET_INFORMATIONW,
    credential: *const CREDENTIALW,
    flags: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredWriteDomainCredentialsW(
                targetinfo: *const CREDENTIAL_TARGET_INFORMATIONW,
                credential: *const CREDENTIALW,
                flags: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CredWriteDomainCredentialsW(
            ::std::mem::transmute(targetinfo),
            ::std::mem::transmute(credential),
            ::std::mem::transmute(flags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredWriteW(
    credential: *const CREDENTIALW,
    flags: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CredWriteW(
                credential: *const CREDENTIALW,
                flags: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CredWriteW(
            ::std::mem::transmute(credential),
            ::std::mem::transmute(flags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const FILE_DEVICE_SMARTCARD: u32 = 49u32;
pub const GUID_DEVINTERFACE_SMARTCARD_READER: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1356681776,
        47754,
        4561,
        [191, 93, 0, 0, 248, 5, 245, 48],
    );
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetOpenCardNameA(param0: *mut OPENCARDNAMEA) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetOpenCardNameA(param0: *mut ::std::mem::ManuallyDrop<OPENCARDNAMEA>) -> i32;
        }
        ::std::mem::transmute(GetOpenCardNameA(::std::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetOpenCardNameW(param0: *mut OPENCARDNAMEW) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetOpenCardNameW(param0: *mut ::std::mem::ManuallyDrop<OPENCARDNAMEW>) -> i32;
        }
        ::std::mem::transmute(GetOpenCardNameW(::std::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn KeyCredentialManagerFreeInformation(
    keycredentialmanagerinfo: *const KeyCredentialManagerInfo,
) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn KeyCredentialManagerFreeInformation(
                keycredentialmanagerinfo: *const KeyCredentialManagerInfo,
            );
        }
        ::std::mem::transmute(KeyCredentialManagerFreeInformation(::std::mem::transmute(
            keycredentialmanagerinfo,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn KeyCredentialManagerGetInformation(
) -> ::windows::runtime::Result<*mut KeyCredentialManagerInfo> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn KeyCredentialManagerGetInformation(
                keycredentialmanagerinfo: *mut *mut KeyCredentialManagerInfo,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <*mut KeyCredentialManagerInfo as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        KeyCredentialManagerGetInformation(&mut result__)
            .from_abi::<*mut KeyCredentialManagerInfo>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn KeyCredentialManagerGetOperationErrorStates(
    keycredentialmanageroperationtype: KeyCredentialManagerOperationType,
    isready: *mut super::super::Foundation::BOOL,
    keycredentialmanageroperationerrorstates: *mut KeyCredentialManagerOperationErrorStates,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn KeyCredentialManagerGetOperationErrorStates(
                keycredentialmanageroperationtype: KeyCredentialManagerOperationType,
                isready: *mut super::super::Foundation::BOOL,
                keycredentialmanageroperationerrorstates : * mut KeyCredentialManagerOperationErrorStates,
            ) -> ::windows::runtime::HRESULT;
        }
        KeyCredentialManagerGetOperationErrorStates(
            ::std::mem::transmute(keycredentialmanageroperationtype),
            ::std::mem::transmute(isready),
            ::std::mem::transmute(keycredentialmanageroperationerrorstates),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct KeyCredentialManagerInfo {
    pub containerId: ::windows::runtime::GUID,
}
impl KeyCredentialManagerInfo {}
impl ::std::default::Default for KeyCredentialManagerInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for KeyCredentialManagerInfo {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("KeyCredentialManagerInfo")
            .field("containerId", &self.containerId)
            .finish()
    }
}
impl ::std::cmp::PartialEq for KeyCredentialManagerInfo {
    fn eq(&self, other: &Self) -> bool {
        self.containerId == other.containerId
    }
}
impl ::std::cmp::Eq for KeyCredentialManagerInfo {}
unsafe impl ::windows::runtime::Abi for KeyCredentialManagerInfo {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct KeyCredentialManagerOperationErrorStates(pub u32);
pub const KeyCredentialManagerOperationErrorStateNone: KeyCredentialManagerOperationErrorStates =
    KeyCredentialManagerOperationErrorStates(0u32);
pub const KeyCredentialManagerOperationErrorStateDeviceJoinFailure:
    KeyCredentialManagerOperationErrorStates = KeyCredentialManagerOperationErrorStates(1u32);
pub const KeyCredentialManagerOperationErrorStateTokenFailure:
    KeyCredentialManagerOperationErrorStates = KeyCredentialManagerOperationErrorStates(2u32);
pub const KeyCredentialManagerOperationErrorStateCertificateFailure:
    KeyCredentialManagerOperationErrorStates = KeyCredentialManagerOperationErrorStates(4u32);
pub const KeyCredentialManagerOperationErrorStateRemoteSessionFailure:
    KeyCredentialManagerOperationErrorStates = KeyCredentialManagerOperationErrorStates(8u32);
pub const KeyCredentialManagerOperationErrorStatePolicyFailure:
    KeyCredentialManagerOperationErrorStates = KeyCredentialManagerOperationErrorStates(16u32);
pub const KeyCredentialManagerOperationErrorStateHardwareFailure:
    KeyCredentialManagerOperationErrorStates = KeyCredentialManagerOperationErrorStates(32u32);
pub const KeyCredentialManagerOperationErrorStatePinExistsFailure:
    KeyCredentialManagerOperationErrorStates = KeyCredentialManagerOperationErrorStates(64u32);
impl ::std::convert::From<u32> for KeyCredentialManagerOperationErrorStates {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KeyCredentialManagerOperationErrorStates {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for KeyCredentialManagerOperationErrorStates {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for KeyCredentialManagerOperationErrorStates {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for KeyCredentialManagerOperationErrorStates {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for KeyCredentialManagerOperationErrorStates {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for KeyCredentialManagerOperationErrorStates {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct KeyCredentialManagerOperationType(pub i32);
pub const KeyCredentialManagerProvisioning: KeyCredentialManagerOperationType =
    KeyCredentialManagerOperationType(0i32);
pub const KeyCredentialManagerPinChange: KeyCredentialManagerOperationType =
    KeyCredentialManagerOperationType(1i32);
pub const KeyCredentialManagerPinReset: KeyCredentialManagerOperationType =
    KeyCredentialManagerOperationType(2i32);
impl ::std::convert::From<i32> for KeyCredentialManagerOperationType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KeyCredentialManagerOperationType {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn KeyCredentialManagerShowUIOperation<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    hwndowner: Param0,
    keycredentialmanageroperationtype: KeyCredentialManagerOperationType,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn KeyCredentialManagerShowUIOperation(
                hwndowner: super::super::Foundation::HWND,
                keycredentialmanageroperationtype: KeyCredentialManagerOperationType,
            ) -> ::windows::runtime::HRESULT;
        }
        KeyCredentialManagerShowUIOperation(
            hwndowner.into_param().abi(),
            ::std::mem::transmute(keycredentialmanageroperationtype),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub type LPOCNCHKPROC = unsafe extern "system" fn(
    param0: usize,
    param1: usize,
    param2: *const ::std::ffi::c_void,
) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type LPOCNCONNPROCA = unsafe extern "system" fn(
    param0: usize,
    param1: super::super::Foundation::PSTR,
    param2: super::super::Foundation::PSTR,
    param3: *const ::std::ffi::c_void,
) -> usize;
#[cfg(feature = "Win32_Foundation")]
pub type LPOCNCONNPROCW = unsafe extern "system" fn(
    param0: usize,
    param1: super::super::Foundation::PWSTR,
    param2: super::super::Foundation::PWSTR,
    param3: *const ::std::ffi::c_void,
) -> usize;
pub type LPOCNDSCPROC =
    unsafe extern "system" fn(param0: usize, param1: usize, param2: *const ::std::ffi::c_void);
pub const MAXIMUM_ATTR_STRING_LENGTH: u32 = 32u32;
pub const MAXIMUM_SMARTCARD_READERS: u32 = 10u32;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct OPENCARDNAMEA {
    pub dwStructSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub hSCardContext: usize,
    pub lpstrGroupNames: super::super::Foundation::PSTR,
    pub nMaxGroupNames: u32,
    pub lpstrCardNames: super::super::Foundation::PSTR,
    pub nMaxCardNames: u32,
    pub rgguidInterfaces: *mut ::windows::runtime::GUID,
    pub cguidInterfaces: u32,
    pub lpstrRdr: super::super::Foundation::PSTR,
    pub nMaxRdr: u32,
    pub lpstrCard: super::super::Foundation::PSTR,
    pub nMaxCard: u32,
    pub lpstrTitle: super::super::Foundation::PSTR,
    pub dwFlags: u32,
    pub pvUserData: *mut ::std::ffi::c_void,
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
    pub dwActiveProtocol: u32,
    pub lpfnConnect: ::std::option::Option<LPOCNCONNPROCA>,
    pub lpfnCheck: ::std::option::Option<LPOCNCHKPROC>,
    pub lpfnDisconnect: ::std::option::Option<LPOCNDSCPROC>,
    pub hCardHandle: usize,
}
#[cfg(feature = "Win32_Foundation")]
impl OPENCARDNAMEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for OPENCARDNAMEA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for OPENCARDNAMEA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OPENCARDNAMEA")
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
            .field("hCardHandle", &self.hCardHandle)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for OPENCARDNAMEA {
    fn eq(&self, other: &Self) -> bool {
        self.dwStructSize == other.dwStructSize
            && self.hwndOwner == other.hwndOwner
            && self.hSCardContext == other.hSCardContext
            && self.lpstrGroupNames == other.lpstrGroupNames
            && self.nMaxGroupNames == other.nMaxGroupNames
            && self.lpstrCardNames == other.lpstrCardNames
            && self.nMaxCardNames == other.nMaxCardNames
            && self.rgguidInterfaces == other.rgguidInterfaces
            && self.cguidInterfaces == other.cguidInterfaces
            && self.lpstrRdr == other.lpstrRdr
            && self.nMaxRdr == other.nMaxRdr
            && self.lpstrCard == other.lpstrCard
            && self.nMaxCard == other.nMaxCard
            && self.lpstrTitle == other.lpstrTitle
            && self.dwFlags == other.dwFlags
            && self.pvUserData == other.pvUserData
            && self.dwShareMode == other.dwShareMode
            && self.dwPreferredProtocols == other.dwPreferredProtocols
            && self.dwActiveProtocol == other.dwActiveProtocol
            && self.lpfnConnect.map(|f| f as usize) == other.lpfnConnect.map(|f| f as usize)
            && self.lpfnCheck.map(|f| f as usize) == other.lpfnCheck.map(|f| f as usize)
            && self.lpfnDisconnect.map(|f| f as usize) == other.lpfnDisconnect.map(|f| f as usize)
            && self.hCardHandle == other.hCardHandle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for OPENCARDNAMEA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for OPENCARDNAMEA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct OPENCARDNAMEW {
    pub dwStructSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub hSCardContext: usize,
    pub lpstrGroupNames: super::super::Foundation::PWSTR,
    pub nMaxGroupNames: u32,
    pub lpstrCardNames: super::super::Foundation::PWSTR,
    pub nMaxCardNames: u32,
    pub rgguidInterfaces: *mut ::windows::runtime::GUID,
    pub cguidInterfaces: u32,
    pub lpstrRdr: super::super::Foundation::PWSTR,
    pub nMaxRdr: u32,
    pub lpstrCard: super::super::Foundation::PWSTR,
    pub nMaxCard: u32,
    pub lpstrTitle: super::super::Foundation::PWSTR,
    pub dwFlags: u32,
    pub pvUserData: *mut ::std::ffi::c_void,
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
    pub dwActiveProtocol: u32,
    pub lpfnConnect: ::std::option::Option<LPOCNCONNPROCW>,
    pub lpfnCheck: ::std::option::Option<LPOCNCHKPROC>,
    pub lpfnDisconnect: ::std::option::Option<LPOCNDSCPROC>,
    pub hCardHandle: usize,
}
#[cfg(feature = "Win32_Foundation")]
impl OPENCARDNAMEW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for OPENCARDNAMEW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for OPENCARDNAMEW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OPENCARDNAMEW")
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
            .field("hCardHandle", &self.hCardHandle)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for OPENCARDNAMEW {
    fn eq(&self, other: &Self) -> bool {
        self.dwStructSize == other.dwStructSize
            && self.hwndOwner == other.hwndOwner
            && self.hSCardContext == other.hSCardContext
            && self.lpstrGroupNames == other.lpstrGroupNames
            && self.nMaxGroupNames == other.nMaxGroupNames
            && self.lpstrCardNames == other.lpstrCardNames
            && self.nMaxCardNames == other.nMaxCardNames
            && self.rgguidInterfaces == other.rgguidInterfaces
            && self.cguidInterfaces == other.cguidInterfaces
            && self.lpstrRdr == other.lpstrRdr
            && self.nMaxRdr == other.nMaxRdr
            && self.lpstrCard == other.lpstrCard
            && self.nMaxCard == other.nMaxCard
            && self.lpstrTitle == other.lpstrTitle
            && self.dwFlags == other.dwFlags
            && self.pvUserData == other.pvUserData
            && self.dwShareMode == other.dwShareMode
            && self.dwPreferredProtocols == other.dwPreferredProtocols
            && self.dwActiveProtocol == other.dwActiveProtocol
            && self.lpfnConnect.map(|f| f as usize) == other.lpfnConnect.map(|f| f as usize)
            && self.lpfnCheck.map(|f| f as usize) == other.lpfnCheck.map(|f| f as usize)
            && self.lpfnDisconnect.map(|f| f as usize) == other.lpfnDisconnect.map(|f| f as usize)
            && self.hCardHandle == other.hCardHandle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for OPENCARDNAMEW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for OPENCARDNAMEW {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
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
    pub lpfnConnect: ::std::option::Option<LPOCNCONNPROCA>,
    pub pvUserData: *mut ::std::ffi::c_void,
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
impl OPENCARDNAME_EXA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for OPENCARDNAME_EXA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::fmt::Debug for OPENCARDNAME_EXA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OPENCARDNAME_EXA")
            .field("dwStructSize", &self.dwStructSize)
            .field("hSCardContext", &self.hSCardContext)
            .field("hwndOwner", &self.hwndOwner)
            .field("dwFlags", &self.dwFlags)
            .field("lpstrTitle", &self.lpstrTitle)
            .field("lpstrSearchDesc", &self.lpstrSearchDesc)
            .field("hIcon", &self.hIcon)
            .field("pOpenCardSearchCriteria", &self.pOpenCardSearchCriteria)
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
impl ::std::cmp::PartialEq for OPENCARDNAME_EXA {
    fn eq(&self, other: &Self) -> bool {
        self.dwStructSize == other.dwStructSize
            && self.hSCardContext == other.hSCardContext
            && self.hwndOwner == other.hwndOwner
            && self.dwFlags == other.dwFlags
            && self.lpstrTitle == other.lpstrTitle
            && self.lpstrSearchDesc == other.lpstrSearchDesc
            && self.hIcon == other.hIcon
            && self.pOpenCardSearchCriteria == other.pOpenCardSearchCriteria
            && self.lpfnConnect.map(|f| f as usize) == other.lpfnConnect.map(|f| f as usize)
            && self.pvUserData == other.pvUserData
            && self.dwShareMode == other.dwShareMode
            && self.dwPreferredProtocols == other.dwPreferredProtocols
            && self.lpstrRdr == other.lpstrRdr
            && self.nMaxRdr == other.nMaxRdr
            && self.lpstrCard == other.lpstrCard
            && self.nMaxCard == other.nMaxCard
            && self.dwActiveProtocol == other.dwActiveProtocol
            && self.hCardHandle == other.hCardHandle
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for OPENCARDNAME_EXA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for OPENCARDNAME_EXA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
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
    pub lpfnConnect: ::std::option::Option<LPOCNCONNPROCW>,
    pub pvUserData: *mut ::std::ffi::c_void,
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
impl OPENCARDNAME_EXW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for OPENCARDNAME_EXW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::fmt::Debug for OPENCARDNAME_EXW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OPENCARDNAME_EXW")
            .field("dwStructSize", &self.dwStructSize)
            .field("hSCardContext", &self.hSCardContext)
            .field("hwndOwner", &self.hwndOwner)
            .field("dwFlags", &self.dwFlags)
            .field("lpstrTitle", &self.lpstrTitle)
            .field("lpstrSearchDesc", &self.lpstrSearchDesc)
            .field("hIcon", &self.hIcon)
            .field("pOpenCardSearchCriteria", &self.pOpenCardSearchCriteria)
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
impl ::std::cmp::PartialEq for OPENCARDNAME_EXW {
    fn eq(&self, other: &Self) -> bool {
        self.dwStructSize == other.dwStructSize
            && self.hSCardContext == other.hSCardContext
            && self.hwndOwner == other.hwndOwner
            && self.dwFlags == other.dwFlags
            && self.lpstrTitle == other.lpstrTitle
            && self.lpstrSearchDesc == other.lpstrSearchDesc
            && self.hIcon == other.hIcon
            && self.pOpenCardSearchCriteria == other.pOpenCardSearchCriteria
            && self.lpfnConnect.map(|f| f as usize) == other.lpfnConnect.map(|f| f as usize)
            && self.pvUserData == other.pvUserData
            && self.dwShareMode == other.dwShareMode
            && self.dwPreferredProtocols == other.dwPreferredProtocols
            && self.lpstrRdr == other.lpstrRdr
            && self.nMaxRdr == other.nMaxRdr
            && self.lpstrCard == other.lpstrCard
            && self.nMaxCard == other.nMaxCard
            && self.dwActiveProtocol == other.dwActiveProtocol
            && self.hCardHandle == other.hCardHandle
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for OPENCARDNAME_EXW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for OPENCARDNAME_EXW {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct OPENCARD_SEARCH_CRITERIAA {
    pub dwStructSize: u32,
    pub lpstrGroupNames: super::super::Foundation::PSTR,
    pub nMaxGroupNames: u32,
    pub rgguidInterfaces: *mut ::windows::runtime::GUID,
    pub cguidInterfaces: u32,
    pub lpstrCardNames: super::super::Foundation::PSTR,
    pub nMaxCardNames: u32,
    pub lpfnCheck: ::std::option::Option<LPOCNCHKPROC>,
    pub lpfnConnect: ::std::option::Option<LPOCNCONNPROCA>,
    pub lpfnDisconnect: ::std::option::Option<LPOCNDSCPROC>,
    pub pvUserData: *mut ::std::ffi::c_void,
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl OPENCARD_SEARCH_CRITERIAA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for OPENCARD_SEARCH_CRITERIAA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for OPENCARD_SEARCH_CRITERIAA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OPENCARD_SEARCH_CRITERIAA")
            .field("dwStructSize", &self.dwStructSize)
            .field("lpstrGroupNames", &self.lpstrGroupNames)
            .field("nMaxGroupNames", &self.nMaxGroupNames)
            .field("rgguidInterfaces", &self.rgguidInterfaces)
            .field("cguidInterfaces", &self.cguidInterfaces)
            .field("lpstrCardNames", &self.lpstrCardNames)
            .field("nMaxCardNames", &self.nMaxCardNames)
            .field("pvUserData", &self.pvUserData)
            .field("dwShareMode", &self.dwShareMode)
            .field("dwPreferredProtocols", &self.dwPreferredProtocols)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for OPENCARD_SEARCH_CRITERIAA {
    fn eq(&self, other: &Self) -> bool {
        self.dwStructSize == other.dwStructSize
            && self.lpstrGroupNames == other.lpstrGroupNames
            && self.nMaxGroupNames == other.nMaxGroupNames
            && self.rgguidInterfaces == other.rgguidInterfaces
            && self.cguidInterfaces == other.cguidInterfaces
            && self.lpstrCardNames == other.lpstrCardNames
            && self.nMaxCardNames == other.nMaxCardNames
            && self.lpfnCheck.map(|f| f as usize) == other.lpfnCheck.map(|f| f as usize)
            && self.lpfnConnect.map(|f| f as usize) == other.lpfnConnect.map(|f| f as usize)
            && self.lpfnDisconnect.map(|f| f as usize) == other.lpfnDisconnect.map(|f| f as usize)
            && self.pvUserData == other.pvUserData
            && self.dwShareMode == other.dwShareMode
            && self.dwPreferredProtocols == other.dwPreferredProtocols
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for OPENCARD_SEARCH_CRITERIAA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for OPENCARD_SEARCH_CRITERIAA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct OPENCARD_SEARCH_CRITERIAW {
    pub dwStructSize: u32,
    pub lpstrGroupNames: super::super::Foundation::PWSTR,
    pub nMaxGroupNames: u32,
    pub rgguidInterfaces: *mut ::windows::runtime::GUID,
    pub cguidInterfaces: u32,
    pub lpstrCardNames: super::super::Foundation::PWSTR,
    pub nMaxCardNames: u32,
    pub lpfnCheck: ::std::option::Option<LPOCNCHKPROC>,
    pub lpfnConnect: ::std::option::Option<LPOCNCONNPROCW>,
    pub lpfnDisconnect: ::std::option::Option<LPOCNDSCPROC>,
    pub pvUserData: *mut ::std::ffi::c_void,
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl OPENCARD_SEARCH_CRITERIAW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for OPENCARD_SEARCH_CRITERIAW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for OPENCARD_SEARCH_CRITERIAW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OPENCARD_SEARCH_CRITERIAW")
            .field("dwStructSize", &self.dwStructSize)
            .field("lpstrGroupNames", &self.lpstrGroupNames)
            .field("nMaxGroupNames", &self.nMaxGroupNames)
            .field("rgguidInterfaces", &self.rgguidInterfaces)
            .field("cguidInterfaces", &self.cguidInterfaces)
            .field("lpstrCardNames", &self.lpstrCardNames)
            .field("nMaxCardNames", &self.nMaxCardNames)
            .field("pvUserData", &self.pvUserData)
            .field("dwShareMode", &self.dwShareMode)
            .field("dwPreferredProtocols", &self.dwPreferredProtocols)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for OPENCARD_SEARCH_CRITERIAW {
    fn eq(&self, other: &Self) -> bool {
        self.dwStructSize == other.dwStructSize
            && self.lpstrGroupNames == other.lpstrGroupNames
            && self.nMaxGroupNames == other.nMaxGroupNames
            && self.rgguidInterfaces == other.rgguidInterfaces
            && self.cguidInterfaces == other.cguidInterfaces
            && self.lpstrCardNames == other.lpstrCardNames
            && self.nMaxCardNames == other.nMaxCardNames
            && self.lpfnCheck.map(|f| f as usize) == other.lpfnCheck.map(|f| f as usize)
            && self.lpfnConnect.map(|f| f as usize) == other.lpfnConnect.map(|f| f as usize)
            && self.lpfnDisconnect.map(|f| f as usize) == other.lpfnDisconnect.map(|f| f as usize)
            && self.pvUserData == other.pvUserData
            && self.dwShareMode == other.dwShareMode
            && self.dwPreferredProtocols == other.dwPreferredProtocols
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for OPENCARD_SEARCH_CRITERIAW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for OPENCARD_SEARCH_CRITERIAW {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct READER_SEL_REQUEST {
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
    pub MatchType: READER_SEL_REQUEST_MATCH_TYPE,
    pub Anonymous: READER_SEL_REQUEST_0,
}
impl READER_SEL_REQUEST {}
impl ::std::default::Default for READER_SEL_REQUEST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for READER_SEL_REQUEST {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for READER_SEL_REQUEST {}
unsafe impl ::windows::runtime::Abi for READER_SEL_REQUEST {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union READER_SEL_REQUEST_0 {
    pub ReaderAndContainerParameter: READER_SEL_REQUEST_0_0,
    pub SerialNumberParameter: READER_SEL_REQUEST_0_1,
}
impl READER_SEL_REQUEST_0 {}
impl ::std::default::Default for READER_SEL_REQUEST_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for READER_SEL_REQUEST_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for READER_SEL_REQUEST_0 {}
unsafe impl ::windows::runtime::Abi for READER_SEL_REQUEST_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct READER_SEL_REQUEST_0_0 {
    pub cbReaderNameOffset: u32,
    pub cchReaderNameLength: u32,
    pub cbContainerNameOffset: u32,
    pub cchContainerNameLength: u32,
    pub dwDesiredCardModuleVersion: u32,
    pub dwCspFlags: u32,
}
impl READER_SEL_REQUEST_0_0 {}
impl ::std::default::Default for READER_SEL_REQUEST_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for READER_SEL_REQUEST_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_ReaderAndContainerParameter_e__Struct")
            .field("cbReaderNameOffset", &self.cbReaderNameOffset)
            .field("cchReaderNameLength", &self.cchReaderNameLength)
            .field("cbContainerNameOffset", &self.cbContainerNameOffset)
            .field("cchContainerNameLength", &self.cchContainerNameLength)
            .field(
                "dwDesiredCardModuleVersion",
                &self.dwDesiredCardModuleVersion,
            )
            .field("dwCspFlags", &self.dwCspFlags)
            .finish()
    }
}
impl ::std::cmp::PartialEq for READER_SEL_REQUEST_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.cbReaderNameOffset == other.cbReaderNameOffset
            && self.cchReaderNameLength == other.cchReaderNameLength
            && self.cbContainerNameOffset == other.cbContainerNameOffset
            && self.cchContainerNameLength == other.cchContainerNameLength
            && self.dwDesiredCardModuleVersion == other.dwDesiredCardModuleVersion
            && self.dwCspFlags == other.dwCspFlags
    }
}
impl ::std::cmp::Eq for READER_SEL_REQUEST_0_0 {}
unsafe impl ::windows::runtime::Abi for READER_SEL_REQUEST_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct READER_SEL_REQUEST_0_1 {
    pub cbSerialNumberOffset: u32,
    pub cbSerialNumberLength: u32,
    pub dwDesiredCardModuleVersion: u32,
}
impl READER_SEL_REQUEST_0_1 {}
impl ::std::default::Default for READER_SEL_REQUEST_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for READER_SEL_REQUEST_0_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_SerialNumberParameter_e__Struct")
            .field("cbSerialNumberOffset", &self.cbSerialNumberOffset)
            .field("cbSerialNumberLength", &self.cbSerialNumberLength)
            .field(
                "dwDesiredCardModuleVersion",
                &self.dwDesiredCardModuleVersion,
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for READER_SEL_REQUEST_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.cbSerialNumberOffset == other.cbSerialNumberOffset
            && self.cbSerialNumberLength == other.cbSerialNumberLength
            && self.dwDesiredCardModuleVersion == other.dwDesiredCardModuleVersion
    }
}
impl ::std::cmp::Eq for READER_SEL_REQUEST_0_1 {}
unsafe impl ::windows::runtime::Abi for READER_SEL_REQUEST_0_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct READER_SEL_REQUEST_MATCH_TYPE(pub i32);
pub const RSR_MATCH_TYPE_READER_AND_CONTAINER: READER_SEL_REQUEST_MATCH_TYPE =
    READER_SEL_REQUEST_MATCH_TYPE(1i32);
pub const RSR_MATCH_TYPE_SERIAL_NUMBER: READER_SEL_REQUEST_MATCH_TYPE =
    READER_SEL_REQUEST_MATCH_TYPE(2i32);
pub const RSR_MATCH_TYPE_ALL_CARDS: READER_SEL_REQUEST_MATCH_TYPE =
    READER_SEL_REQUEST_MATCH_TYPE(3i32);
impl ::std::convert::From<i32> for READER_SEL_REQUEST_MATCH_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for READER_SEL_REQUEST_MATCH_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct READER_SEL_RESPONSE {
    pub cbReaderNameOffset: u32,
    pub cchReaderNameLength: u32,
    pub cbCardNameOffset: u32,
    pub cchCardNameLength: u32,
}
impl READER_SEL_RESPONSE {}
impl ::std::default::Default for READER_SEL_RESPONSE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for READER_SEL_RESPONSE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("READER_SEL_RESPONSE")
            .field("cbReaderNameOffset", &self.cbReaderNameOffset)
            .field("cchReaderNameLength", &self.cchReaderNameLength)
            .field("cbCardNameOffset", &self.cbCardNameOffset)
            .field("cchCardNameLength", &self.cchCardNameLength)
            .finish()
    }
}
impl ::std::cmp::PartialEq for READER_SEL_RESPONSE {
    fn eq(&self, other: &Self) -> bool {
        self.cbReaderNameOffset == other.cbReaderNameOffset
            && self.cchReaderNameLength == other.cchReaderNameLength
            && self.cbCardNameOffset == other.cbCardNameOffset
            && self.cchCardNameLength == other.cchCardNameLength
    }
}
impl ::std::cmp::Eq for READER_SEL_RESPONSE {}
unsafe impl ::windows::runtime::Abi for READER_SEL_RESPONSE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const SCARD_ABSENT: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SCARD_ATRMASK {
    pub cbAtr: u32,
    pub rgbAtr: [u8; 36],
    pub rgbMask: [u8; 36],
}
impl SCARD_ATRMASK {}
impl ::std::default::Default for SCARD_ATRMASK {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SCARD_ATRMASK {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SCARD_ATRMASK")
            .field("cbAtr", &self.cbAtr)
            .field("rgbAtr", &self.rgbAtr)
            .field("rgbMask", &self.rgbMask)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SCARD_ATRMASK {
    fn eq(&self, other: &Self) -> bool {
        self.cbAtr == other.cbAtr && self.rgbAtr == other.rgbAtr && self.rgbMask == other.rgbMask
    }
}
impl ::std::cmp::Eq for SCARD_ATRMASK {}
unsafe impl ::windows::runtime::Abi for SCARD_ATRMASK {
    type Abi = Self;
    type DefaultType = Self;
}
pub const SCARD_ATR_LENGTH: u32 = 33u32;
pub const SCARD_AUDIT_CHV_FAILURE: u32 = 0u32;
pub const SCARD_AUDIT_CHV_SUCCESS: u32 = 1u32;
pub const SCARD_CLASS_COMMUNICATIONS: u32 = 2u32;
pub const SCARD_CLASS_ICC_STATE: u32 = 9u32;
pub const SCARD_CLASS_IFD_PROTOCOL: u32 = 8u32;
pub const SCARD_CLASS_MECHANICAL: u32 = 6u32;
pub const SCARD_CLASS_PERF: u32 = 32766u32;
pub const SCARD_CLASS_POWER_MGMT: u32 = 4u32;
pub const SCARD_CLASS_PROTOCOL: u32 = 3u32;
pub const SCARD_CLASS_SECURITY: u32 = 5u32;
pub const SCARD_CLASS_SYSTEM: u32 = 32767u32;
pub const SCARD_CLASS_VENDOR_DEFINED: u32 = 7u32;
pub const SCARD_CLASS_VENDOR_INFO: u32 = 1u32;
pub const SCARD_COLD_RESET: u32 = 1u32;
pub const SCARD_EJECT_CARD: u32 = 3u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SCARD_IO_REQUEST {
    pub dwProtocol: u32,
    pub cbPciLength: u32,
}
impl SCARD_IO_REQUEST {}
impl ::std::default::Default for SCARD_IO_REQUEST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SCARD_IO_REQUEST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SCARD_IO_REQUEST")
            .field("dwProtocol", &self.dwProtocol)
            .field("cbPciLength", &self.cbPciLength)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SCARD_IO_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.dwProtocol == other.dwProtocol && self.cbPciLength == other.cbPciLength
    }
}
impl ::std::cmp::Eq for SCARD_IO_REQUEST {}
unsafe impl ::windows::runtime::Abi for SCARD_IO_REQUEST {
    type Abi = Self;
    type DefaultType = Self;
}
pub const SCARD_LEAVE_CARD: u32 = 0u32;
pub const SCARD_NEGOTIABLE: u32 = 5u32;
pub const SCARD_POWERED: u32 = 4u32;
pub const SCARD_POWER_DOWN: u32 = 0u32;
pub const SCARD_PRESENT: u32 = 2u32;
pub const SCARD_PROTOCOL_DEFAULT: u32 = 2147483648u32;
pub const SCARD_PROTOCOL_OPTIMAL: u32 = 0u32;
pub const SCARD_PROTOCOL_RAW: u32 = 65536u32;
pub const SCARD_PROTOCOL_T0: u32 = 1u32;
pub const SCARD_PROTOCOL_T1: u32 = 2u32;
pub const SCARD_PROTOCOL_UNDEFINED: u32 = 0u32;
pub const SCARD_PROVIDER_CSP: u32 = 2u32;
pub const SCARD_PROVIDER_KSP: u32 = 3u32;
pub const SCARD_PROVIDER_PRIMARY: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SCARD_READERSTATEA {
    pub szReader: super::super::Foundation::PSTR,
    pub pvUserData: *mut ::std::ffi::c_void,
    pub dwCurrentState: SCARD_STATE,
    pub dwEventState: SCARD_STATE,
    pub cbAtr: u32,
    pub rgbAtr: [u8; 36],
}
#[cfg(feature = "Win32_Foundation")]
impl SCARD_READERSTATEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SCARD_READERSTATEA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SCARD_READERSTATEA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SCARD_READERSTATEA")
            .field("szReader", &self.szReader)
            .field("pvUserData", &self.pvUserData)
            .field("dwCurrentState", &self.dwCurrentState)
            .field("dwEventState", &self.dwEventState)
            .field("cbAtr", &self.cbAtr)
            .field("rgbAtr", &self.rgbAtr)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SCARD_READERSTATEA {
    fn eq(&self, other: &Self) -> bool {
        self.szReader == other.szReader
            && self.pvUserData == other.pvUserData
            && self.dwCurrentState == other.dwCurrentState
            && self.dwEventState == other.dwEventState
            && self.cbAtr == other.cbAtr
            && self.rgbAtr == other.rgbAtr
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SCARD_READERSTATEA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SCARD_READERSTATEA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SCARD_READERSTATEW {
    pub szReader: super::super::Foundation::PWSTR,
    pub pvUserData: *mut ::std::ffi::c_void,
    pub dwCurrentState: SCARD_STATE,
    pub dwEventState: SCARD_STATE,
    pub cbAtr: u32,
    pub rgbAtr: [u8; 36],
}
#[cfg(feature = "Win32_Foundation")]
impl SCARD_READERSTATEW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SCARD_READERSTATEW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SCARD_READERSTATEW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SCARD_READERSTATEW")
            .field("szReader", &self.szReader)
            .field("pvUserData", &self.pvUserData)
            .field("dwCurrentState", &self.dwCurrentState)
            .field("dwEventState", &self.dwEventState)
            .field("cbAtr", &self.cbAtr)
            .field("rgbAtr", &self.rgbAtr)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SCARD_READERSTATEW {
    fn eq(&self, other: &Self) -> bool {
        self.szReader == other.szReader
            && self.pvUserData == other.pvUserData
            && self.dwCurrentState == other.dwCurrentState
            && self.dwEventState == other.dwEventState
            && self.cbAtr == other.cbAtr
            && self.rgbAtr == other.rgbAtr
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SCARD_READERSTATEW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SCARD_READERSTATEW {
    type Abi = Self;
    type DefaultType = Self;
}
pub const SCARD_READER_CONFISCATES: u32 = 4u32;
pub const SCARD_READER_CONTACTLESS: u32 = 8u32;
pub const SCARD_READER_EJECTS: u32 = 2u32;
pub const SCARD_READER_SWALLOWS: u32 = 1u32;
pub const SCARD_READER_TYPE_EMBEDDEDSE: u32 = 2048u32;
pub const SCARD_READER_TYPE_IDE: u32 = 16u32;
pub const SCARD_READER_TYPE_KEYBOARD: u32 = 4u32;
pub const SCARD_READER_TYPE_NFC: u32 = 256u32;
pub const SCARD_READER_TYPE_NGC: u32 = 1024u32;
pub const SCARD_READER_TYPE_PARALELL: u32 = 2u32;
pub const SCARD_READER_TYPE_PCMCIA: u32 = 64u32;
pub const SCARD_READER_TYPE_SCSI: u32 = 8u32;
pub const SCARD_READER_TYPE_SERIAL: u32 = 1u32;
pub const SCARD_READER_TYPE_TPM: u32 = 128u32;
pub const SCARD_READER_TYPE_UICC: u32 = 512u32;
pub const SCARD_READER_TYPE_USB: u32 = 32u32;
pub const SCARD_READER_TYPE_VENDOR: u32 = 240u32;
pub const SCARD_RESET_CARD: u32 = 1u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct SCARD_SCOPE(pub u32);
pub const SCARD_SCOPE_USER: SCARD_SCOPE = SCARD_SCOPE(0u32);
pub const SCARD_SCOPE_SYSTEM: SCARD_SCOPE = SCARD_SCOPE(2u32);
impl ::std::convert::From<u32> for SCARD_SCOPE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SCARD_SCOPE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for SCARD_SCOPE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for SCARD_SCOPE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for SCARD_SCOPE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for SCARD_SCOPE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for SCARD_SCOPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const SCARD_SCOPE_TERMINAL: u32 = 1u32;
pub const SCARD_SHARE_DIRECT: u32 = 3u32;
pub const SCARD_SHARE_EXCLUSIVE: u32 = 1u32;
pub const SCARD_SHARE_SHARED: u32 = 2u32;
pub const SCARD_SPECIFIC: u32 = 6u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct SCARD_STATE(pub u32);
pub const SCARD_STATE_UNAWARE: SCARD_STATE = SCARD_STATE(0u32);
pub const SCARD_STATE_IGNORE: SCARD_STATE = SCARD_STATE(1u32);
pub const SCARD_STATE_UNAVAILABLE: SCARD_STATE = SCARD_STATE(8u32);
pub const SCARD_STATE_EMPTY: SCARD_STATE = SCARD_STATE(16u32);
pub const SCARD_STATE_PRESENT: SCARD_STATE = SCARD_STATE(32u32);
pub const SCARD_STATE_ATRMATCH: SCARD_STATE = SCARD_STATE(64u32);
pub const SCARD_STATE_EXCLUSIVE: SCARD_STATE = SCARD_STATE(128u32);
pub const SCARD_STATE_INUSE: SCARD_STATE = SCARD_STATE(256u32);
pub const SCARD_STATE_MUTE: SCARD_STATE = SCARD_STATE(512u32);
pub const SCARD_STATE_CHANGED: SCARD_STATE = SCARD_STATE(2u32);
pub const SCARD_STATE_UNKNOWN: SCARD_STATE = SCARD_STATE(4u32);
impl ::std::convert::From<u32> for SCARD_STATE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SCARD_STATE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for SCARD_STATE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for SCARD_STATE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for SCARD_STATE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for SCARD_STATE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for SCARD_STATE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const SCARD_STATE_UNPOWERED: u32 = 1024u32;
pub const SCARD_SWALLOWED: u32 = 3u32;
pub const SCARD_T0_CMD_LENGTH: u32 = 5u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SCARD_T0_COMMAND {
    pub bCla: u8,
    pub bIns: u8,
    pub bP1: u8,
    pub bP2: u8,
    pub bP3: u8,
}
impl SCARD_T0_COMMAND {}
impl ::std::default::Default for SCARD_T0_COMMAND {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SCARD_T0_COMMAND {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SCARD_T0_COMMAND")
            .field("bCla", &self.bCla)
            .field("bIns", &self.bIns)
            .field("bP1", &self.bP1)
            .field("bP2", &self.bP2)
            .field("bP3", &self.bP3)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SCARD_T0_COMMAND {
    fn eq(&self, other: &Self) -> bool {
        self.bCla == other.bCla
            && self.bIns == other.bIns
            && self.bP1 == other.bP1
            && self.bP2 == other.bP2
            && self.bP3 == other.bP3
    }
}
impl ::std::cmp::Eq for SCARD_T0_COMMAND {}
unsafe impl ::windows::runtime::Abi for SCARD_T0_COMMAND {
    type Abi = Self;
    type DefaultType = Self;
}
pub const SCARD_T0_HEADER_LENGTH: u32 = 7u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SCARD_T0_REQUEST {
    pub ioRequest: SCARD_IO_REQUEST,
    pub bSw1: u8,
    pub bSw2: u8,
    pub Anonymous: SCARD_T0_REQUEST_0,
}
impl SCARD_T0_REQUEST {}
impl ::std::default::Default for SCARD_T0_REQUEST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for SCARD_T0_REQUEST {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for SCARD_T0_REQUEST {}
unsafe impl ::windows::runtime::Abi for SCARD_T0_REQUEST {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union SCARD_T0_REQUEST_0 {
    pub CmdBytes: SCARD_T0_COMMAND,
    pub rgbHeader: [u8; 5],
}
impl SCARD_T0_REQUEST_0 {}
impl ::std::default::Default for SCARD_T0_REQUEST_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for SCARD_T0_REQUEST_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for SCARD_T0_REQUEST_0 {}
unsafe impl ::windows::runtime::Abi for SCARD_T0_REQUEST_0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const SCARD_T1_EPILOGUE_LENGTH: u32 = 2u32;
pub const SCARD_T1_EPILOGUE_LENGTH_LRC: u32 = 1u32;
pub const SCARD_T1_MAX_IFS: u32 = 254u32;
pub const SCARD_T1_PROLOGUE_LENGTH: u32 = 3u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SCARD_T1_REQUEST {
    pub ioRequest: SCARD_IO_REQUEST,
}
impl SCARD_T1_REQUEST {}
impl ::std::default::Default for SCARD_T1_REQUEST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SCARD_T1_REQUEST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SCARD_T1_REQUEST")
            .field("ioRequest", &self.ioRequest)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SCARD_T1_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.ioRequest == other.ioRequest
    }
}
impl ::std::cmp::Eq for SCARD_T1_REQUEST {}
unsafe impl ::windows::runtime::Abi for SCARD_T1_REQUEST {
    type Abi = Self;
    type DefaultType = Self;
}
pub const SCARD_UNKNOWN: u32 = 0u32;
pub const SCARD_UNPOWER_CARD: u32 = 2u32;
pub const SCARD_WARM_RESET: u32 = 2u32;
pub const SCERR_NOCARDNAME: u32 = 16384u32;
pub const SCERR_NOGUIDS: u32 = 32768u32;
pub const SC_DLG_FORCE_UI: u32 = 4u32;
pub const SC_DLG_MINIMAL_UI: u32 = 1u32;
pub const SC_DLG_NO_UI: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardAccessStartedEvent() -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardAccessStartedEvent() -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(SCardAccessStartedEvent())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardAddReaderToGroupA<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hcontext: usize,
    szreadername: Param1,
    szgroupname: Param2,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardAddReaderToGroupA(
                hcontext: usize,
                szreadername: super::super::Foundation::PSTR,
                szgroupname: super::super::Foundation::PSTR,
            ) -> i32;
        }
        ::std::mem::transmute(SCardAddReaderToGroupA(
            ::std::mem::transmute(hcontext),
            szreadername.into_param().abi(),
            szgroupname.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardAddReaderToGroupW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hcontext: usize,
    szreadername: Param1,
    szgroupname: Param2,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardAddReaderToGroupW(
                hcontext: usize,
                szreadername: super::super::Foundation::PWSTR,
                szgroupname: super::super::Foundation::PWSTR,
            ) -> i32;
        }
        ::std::mem::transmute(SCardAddReaderToGroupW(
            ::std::mem::transmute(hcontext),
            szreadername.into_param().abi(),
            szgroupname.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SCardAudit(hcontext: usize, dwevent: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardAudit(hcontext: usize, dwevent: u32) -> i32;
        }
        ::std::mem::transmute(SCardAudit(
            ::std::mem::transmute(hcontext),
            ::std::mem::transmute(dwevent),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SCardBeginTransaction(hcard: usize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardBeginTransaction(hcard: usize) -> i32;
        }
        ::std::mem::transmute(SCardBeginTransaction(::std::mem::transmute(hcard)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SCardCancel(hcontext: usize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardCancel(hcontext: usize) -> i32;
        }
        ::std::mem::transmute(SCardCancel(::std::mem::transmute(hcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardConnectA<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hcontext: usize,
    szreader: Param1,
    dwsharemode: u32,
    dwpreferredprotocols: u32,
    phcard: *mut usize,
    pdwactiveprotocol: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardConnectA(
                hcontext: usize,
                szreader: super::super::Foundation::PSTR,
                dwsharemode: u32,
                dwpreferredprotocols: u32,
                phcard: *mut usize,
                pdwactiveprotocol: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(SCardConnectA(
            ::std::mem::transmute(hcontext),
            szreader.into_param().abi(),
            ::std::mem::transmute(dwsharemode),
            ::std::mem::transmute(dwpreferredprotocols),
            ::std::mem::transmute(phcard),
            ::std::mem::transmute(pdwactiveprotocol),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardConnectW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hcontext: usize,
    szreader: Param1,
    dwsharemode: u32,
    dwpreferredprotocols: u32,
    phcard: *mut usize,
    pdwactiveprotocol: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardConnectW(
                hcontext: usize,
                szreader: super::super::Foundation::PWSTR,
                dwsharemode: u32,
                dwpreferredprotocols: u32,
                phcard: *mut usize,
                pdwactiveprotocol: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(SCardConnectW(
            ::std::mem::transmute(hcontext),
            szreader.into_param().abi(),
            ::std::mem::transmute(dwsharemode),
            ::std::mem::transmute(dwpreferredprotocols),
            ::std::mem::transmute(phcard),
            ::std::mem::transmute(pdwactiveprotocol),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SCardControl(
    hcard: usize,
    dwcontrolcode: u32,
    lpinbuffer: *const ::std::ffi::c_void,
    cbinbuffersize: u32,
    lpoutbuffer: *mut ::std::ffi::c_void,
    cboutbuffersize: u32,
    lpbytesreturned: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardControl(
                hcard: usize,
                dwcontrolcode: u32,
                lpinbuffer: *const ::std::ffi::c_void,
                cbinbuffersize: u32,
                lpoutbuffer: *mut ::std::ffi::c_void,
                cboutbuffersize: u32,
                lpbytesreturned: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(SCardControl(
            ::std::mem::transmute(hcard),
            ::std::mem::transmute(dwcontrolcode),
            ::std::mem::transmute(lpinbuffer),
            ::std::mem::transmute(cbinbuffersize),
            ::std::mem::transmute(lpoutbuffer),
            ::std::mem::transmute(cboutbuffersize),
            ::std::mem::transmute(lpbytesreturned),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SCardDisconnect(hcard: usize, dwdisposition: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardDisconnect(hcard: usize, dwdisposition: u32) -> i32;
        }
        ::std::mem::transmute(SCardDisconnect(
            ::std::mem::transmute(hcard),
            ::std::mem::transmute(dwdisposition),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SCardDlgExtendedError() -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardDlgExtendedError() -> i32;
        }
        ::std::mem::transmute(SCardDlgExtendedError())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SCardEndTransaction(hcard: usize, dwdisposition: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardEndTransaction(hcard: usize, dwdisposition: u32) -> i32;
        }
        ::std::mem::transmute(SCardEndTransaction(
            ::std::mem::transmute(hcard),
            ::std::mem::transmute(dwdisposition),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SCardEstablishContext(
    dwscope: SCARD_SCOPE,
    pvreserved1: *const ::std::ffi::c_void,
    pvreserved2: *const ::std::ffi::c_void,
    phcontext: *mut usize,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardEstablishContext(
                dwscope: SCARD_SCOPE,
                pvreserved1: *const ::std::ffi::c_void,
                pvreserved2: *const ::std::ffi::c_void,
                phcontext: *mut usize,
            ) -> i32;
        }
        ::std::mem::transmute(SCardEstablishContext(
            ::std::mem::transmute(dwscope),
            ::std::mem::transmute(pvreserved1),
            ::std::mem::transmute(pvreserved2),
            ::std::mem::transmute(phcontext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardForgetCardTypeA<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hcontext: usize,
    szcardname: Param1,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardForgetCardTypeA(
                hcontext: usize,
                szcardname: super::super::Foundation::PSTR,
            ) -> i32;
        }
        ::std::mem::transmute(SCardForgetCardTypeA(
            ::std::mem::transmute(hcontext),
            szcardname.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardForgetCardTypeW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hcontext: usize,
    szcardname: Param1,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardForgetCardTypeW(
                hcontext: usize,
                szcardname: super::super::Foundation::PWSTR,
            ) -> i32;
        }
        ::std::mem::transmute(SCardForgetCardTypeW(
            ::std::mem::transmute(hcontext),
            szcardname.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardForgetReaderA<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hcontext: usize,
    szreadername: Param1,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardForgetReaderA(
                hcontext: usize,
                szreadername: super::super::Foundation::PSTR,
            ) -> i32;
        }
        ::std::mem::transmute(SCardForgetReaderA(
            ::std::mem::transmute(hcontext),
            szreadername.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardForgetReaderGroupA<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hcontext: usize,
    szgroupname: Param1,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardForgetReaderGroupA(
                hcontext: usize,
                szgroupname: super::super::Foundation::PSTR,
            ) -> i32;
        }
        ::std::mem::transmute(SCardForgetReaderGroupA(
            ::std::mem::transmute(hcontext),
            szgroupname.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardForgetReaderGroupW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hcontext: usize,
    szgroupname: Param1,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardForgetReaderGroupW(
                hcontext: usize,
                szgroupname: super::super::Foundation::PWSTR,
            ) -> i32;
        }
        ::std::mem::transmute(SCardForgetReaderGroupW(
            ::std::mem::transmute(hcontext),
            szgroupname.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardForgetReaderW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hcontext: usize,
    szreadername: Param1,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardForgetReaderW(
                hcontext: usize,
                szreadername: super::super::Foundation::PWSTR,
            ) -> i32;
        }
        ::std::mem::transmute(SCardForgetReaderW(
            ::std::mem::transmute(hcontext),
            szreadername.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SCardFreeMemory(hcontext: usize, pvmem: *const ::std::ffi::c_void) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardFreeMemory(hcontext: usize, pvmem: *const ::std::ffi::c_void) -> i32;
        }
        ::std::mem::transmute(SCardFreeMemory(
            ::std::mem::transmute(hcontext),
            ::std::mem::transmute(pvmem),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SCardGetAttrib(
    hcard: usize,
    dwattrid: u32,
    pbattr: *mut u8,
    pcbattrlen: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardGetAttrib(
                hcard: usize,
                dwattrid: u32,
                pbattr: *mut u8,
                pcbattrlen: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(SCardGetAttrib(
            ::std::mem::transmute(hcard),
            ::std::mem::transmute(dwattrid),
            ::std::mem::transmute(pbattr),
            ::std::mem::transmute(pcbattrlen),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardGetCardTypeProviderNameA<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hcontext: usize,
    szcardname: Param1,
    dwproviderid: u32,
    szprovider: super::super::Foundation::PSTR,
    pcchprovider: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardGetCardTypeProviderNameA(
                hcontext: usize,
                szcardname: super::super::Foundation::PSTR,
                dwproviderid: u32,
                szprovider: super::super::Foundation::PSTR,
                pcchprovider: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(SCardGetCardTypeProviderNameA(
            ::std::mem::transmute(hcontext),
            szcardname.into_param().abi(),
            ::std::mem::transmute(dwproviderid),
            ::std::mem::transmute(szprovider),
            ::std::mem::transmute(pcchprovider),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardGetCardTypeProviderNameW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hcontext: usize,
    szcardname: Param1,
    dwproviderid: u32,
    szprovider: super::super::Foundation::PWSTR,
    pcchprovider: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardGetCardTypeProviderNameW(
                hcontext: usize,
                szcardname: super::super::Foundation::PWSTR,
                dwproviderid: u32,
                szprovider: super::super::Foundation::PWSTR,
                pcchprovider: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(SCardGetCardTypeProviderNameW(
            ::std::mem::transmute(hcontext),
            szcardname.into_param().abi(),
            ::std::mem::transmute(dwproviderid),
            ::std::mem::transmute(szprovider),
            ::std::mem::transmute(pcchprovider),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardGetDeviceTypeIdA<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hcontext: usize,
    szreadername: Param1,
    pdwdevicetypeid: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardGetDeviceTypeIdA(
                hcontext: usize,
                szreadername: super::super::Foundation::PSTR,
                pdwdevicetypeid: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(SCardGetDeviceTypeIdA(
            ::std::mem::transmute(hcontext),
            szreadername.into_param().abi(),
            ::std::mem::transmute(pdwdevicetypeid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardGetDeviceTypeIdW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hcontext: usize,
    szreadername: Param1,
    pdwdevicetypeid: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardGetDeviceTypeIdW(
                hcontext: usize,
                szreadername: super::super::Foundation::PWSTR,
                pdwdevicetypeid: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(SCardGetDeviceTypeIdW(
            ::std::mem::transmute(hcontext),
            szreadername.into_param().abi(),
            ::std::mem::transmute(pdwdevicetypeid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardGetProviderIdA<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hcontext: usize,
    szcard: Param1,
    pguidproviderid: *mut ::windows::runtime::GUID,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardGetProviderIdA(
                hcontext: usize,
                szcard: super::super::Foundation::PSTR,
                pguidproviderid: *mut ::windows::runtime::GUID,
            ) -> i32;
        }
        ::std::mem::transmute(SCardGetProviderIdA(
            ::std::mem::transmute(hcontext),
            szcard.into_param().abi(),
            ::std::mem::transmute(pguidproviderid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardGetProviderIdW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hcontext: usize,
    szcard: Param1,
    pguidproviderid: *mut ::windows::runtime::GUID,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardGetProviderIdW(
                hcontext: usize,
                szcard: super::super::Foundation::PWSTR,
                pguidproviderid: *mut ::windows::runtime::GUID,
            ) -> i32;
        }
        ::std::mem::transmute(SCardGetProviderIdW(
            ::std::mem::transmute(hcontext),
            szcard.into_param().abi(),
            ::std::mem::transmute(pguidproviderid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardGetReaderDeviceInstanceIdA<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hcontext: usize,
    szreadername: Param1,
    szdeviceinstanceid: super::super::Foundation::PSTR,
    pcchdeviceinstanceid: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardGetReaderDeviceInstanceIdA(
                hcontext: usize,
                szreadername: super::super::Foundation::PSTR,
                szdeviceinstanceid: super::super::Foundation::PSTR,
                pcchdeviceinstanceid: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(SCardGetReaderDeviceInstanceIdA(
            ::std::mem::transmute(hcontext),
            szreadername.into_param().abi(),
            ::std::mem::transmute(szdeviceinstanceid),
            ::std::mem::transmute(pcchdeviceinstanceid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardGetReaderDeviceInstanceIdW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hcontext: usize,
    szreadername: Param1,
    szdeviceinstanceid: super::super::Foundation::PWSTR,
    pcchdeviceinstanceid: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardGetReaderDeviceInstanceIdW(
                hcontext: usize,
                szreadername: super::super::Foundation::PWSTR,
                szdeviceinstanceid: super::super::Foundation::PWSTR,
                pcchdeviceinstanceid: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(SCardGetReaderDeviceInstanceIdW(
            ::std::mem::transmute(hcontext),
            szreadername.into_param().abi(),
            ::std::mem::transmute(szdeviceinstanceid),
            ::std::mem::transmute(pcchdeviceinstanceid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardGetReaderIconA<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hcontext: usize,
    szreadername: Param1,
    pbicon: *mut u8,
    pcbicon: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardGetReaderIconA(
                hcontext: usize,
                szreadername: super::super::Foundation::PSTR,
                pbicon: *mut u8,
                pcbicon: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(SCardGetReaderIconA(
            ::std::mem::transmute(hcontext),
            szreadername.into_param().abi(),
            ::std::mem::transmute(pbicon),
            ::std::mem::transmute(pcbicon),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardGetReaderIconW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hcontext: usize,
    szreadername: Param1,
    pbicon: *mut u8,
    pcbicon: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardGetReaderIconW(
                hcontext: usize,
                szreadername: super::super::Foundation::PWSTR,
                pbicon: *mut u8,
                pcbicon: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(SCardGetReaderIconW(
            ::std::mem::transmute(hcontext),
            szreadername.into_param().abi(),
            ::std::mem::transmute(pbicon),
            ::std::mem::transmute(pcbicon),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardGetStatusChangeA(
    hcontext: usize,
    dwtimeout: u32,
    rgreaderstates: *mut SCARD_READERSTATEA,
    creaders: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardGetStatusChangeA(
                hcontext: usize,
                dwtimeout: u32,
                rgreaderstates: *mut SCARD_READERSTATEA,
                creaders: u32,
            ) -> i32;
        }
        ::std::mem::transmute(SCardGetStatusChangeA(
            ::std::mem::transmute(hcontext),
            ::std::mem::transmute(dwtimeout),
            ::std::mem::transmute(rgreaderstates),
            ::std::mem::transmute(creaders),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardGetStatusChangeW(
    hcontext: usize,
    dwtimeout: u32,
    rgreaderstates: *mut SCARD_READERSTATEW,
    creaders: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardGetStatusChangeW(
                hcontext: usize,
                dwtimeout: u32,
                rgreaderstates: *mut SCARD_READERSTATEW,
                creaders: u32,
            ) -> i32;
        }
        ::std::mem::transmute(SCardGetStatusChangeW(
            ::std::mem::transmute(hcontext),
            ::std::mem::transmute(dwtimeout),
            ::std::mem::transmute(rgreaderstates),
            ::std::mem::transmute(creaders),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SCardGetTransmitCount(hcard: usize, pctransmitcount: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardGetTransmitCount(hcard: usize, pctransmitcount: *mut u32) -> i32;
        }
        ::std::mem::transmute(SCardGetTransmitCount(
            ::std::mem::transmute(hcard),
            ::std::mem::transmute(pctransmitcount),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardIntroduceCardTypeA<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hcontext: usize,
    szcardname: Param1,
    pguidprimaryprovider: *const ::windows::runtime::GUID,
    rgguidinterfaces: *const ::windows::runtime::GUID,
    dwinterfacecount: u32,
    pbatr: *const u8,
    pbatrmask: *const u8,
    cbatrlen: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardIntroduceCardTypeA(
                hcontext: usize,
                szcardname: super::super::Foundation::PSTR,
                pguidprimaryprovider: *const ::windows::runtime::GUID,
                rgguidinterfaces: *const ::windows::runtime::GUID,
                dwinterfacecount: u32,
                pbatr: *const u8,
                pbatrmask: *const u8,
                cbatrlen: u32,
            ) -> i32;
        }
        ::std::mem::transmute(SCardIntroduceCardTypeA(
            ::std::mem::transmute(hcontext),
            szcardname.into_param().abi(),
            ::std::mem::transmute(pguidprimaryprovider),
            ::std::mem::transmute(rgguidinterfaces),
            ::std::mem::transmute(dwinterfacecount),
            ::std::mem::transmute(pbatr),
            ::std::mem::transmute(pbatrmask),
            ::std::mem::transmute(cbatrlen),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardIntroduceCardTypeW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hcontext: usize,
    szcardname: Param1,
    pguidprimaryprovider: *const ::windows::runtime::GUID,
    rgguidinterfaces: *const ::windows::runtime::GUID,
    dwinterfacecount: u32,
    pbatr: *const u8,
    pbatrmask: *const u8,
    cbatrlen: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardIntroduceCardTypeW(
                hcontext: usize,
                szcardname: super::super::Foundation::PWSTR,
                pguidprimaryprovider: *const ::windows::runtime::GUID,
                rgguidinterfaces: *const ::windows::runtime::GUID,
                dwinterfacecount: u32,
                pbatr: *const u8,
                pbatrmask: *const u8,
                cbatrlen: u32,
            ) -> i32;
        }
        ::std::mem::transmute(SCardIntroduceCardTypeW(
            ::std::mem::transmute(hcontext),
            szcardname.into_param().abi(),
            ::std::mem::transmute(pguidprimaryprovider),
            ::std::mem::transmute(rgguidinterfaces),
            ::std::mem::transmute(dwinterfacecount),
            ::std::mem::transmute(pbatr),
            ::std::mem::transmute(pbatrmask),
            ::std::mem::transmute(cbatrlen),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardIntroduceReaderA<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hcontext: usize,
    szreadername: Param1,
    szdevicename: Param2,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardIntroduceReaderA(
                hcontext: usize,
                szreadername: super::super::Foundation::PSTR,
                szdevicename: super::super::Foundation::PSTR,
            ) -> i32;
        }
        ::std::mem::transmute(SCardIntroduceReaderA(
            ::std::mem::transmute(hcontext),
            szreadername.into_param().abi(),
            szdevicename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardIntroduceReaderGroupA<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hcontext: usize,
    szgroupname: Param1,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardIntroduceReaderGroupA(
                hcontext: usize,
                szgroupname: super::super::Foundation::PSTR,
            ) -> i32;
        }
        ::std::mem::transmute(SCardIntroduceReaderGroupA(
            ::std::mem::transmute(hcontext),
            szgroupname.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardIntroduceReaderGroupW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hcontext: usize,
    szgroupname: Param1,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardIntroduceReaderGroupW(
                hcontext: usize,
                szgroupname: super::super::Foundation::PWSTR,
            ) -> i32;
        }
        ::std::mem::transmute(SCardIntroduceReaderGroupW(
            ::std::mem::transmute(hcontext),
            szgroupname.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardIntroduceReaderW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hcontext: usize,
    szreadername: Param1,
    szdevicename: Param2,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardIntroduceReaderW(
                hcontext: usize,
                szreadername: super::super::Foundation::PWSTR,
                szdevicename: super::super::Foundation::PWSTR,
            ) -> i32;
        }
        ::std::mem::transmute(SCardIntroduceReaderW(
            ::std::mem::transmute(hcontext),
            szreadername.into_param().abi(),
            szdevicename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SCardIsValidContext(hcontext: usize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardIsValidContext(hcontext: usize) -> i32;
        }
        ::std::mem::transmute(SCardIsValidContext(::std::mem::transmute(hcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardListCardsA(
    hcontext: usize,
    pbatr: *const u8,
    rgquidinterfaces: *const ::windows::runtime::GUID,
    cguidinterfacecount: u32,
    mszcards: super::super::Foundation::PSTR,
    pcchcards: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardListCardsA(
                hcontext: usize,
                pbatr: *const u8,
                rgquidinterfaces: *const ::windows::runtime::GUID,
                cguidinterfacecount: u32,
                mszcards: super::super::Foundation::PSTR,
                pcchcards: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(SCardListCardsA(
            ::std::mem::transmute(hcontext),
            ::std::mem::transmute(pbatr),
            ::std::mem::transmute(rgquidinterfaces),
            ::std::mem::transmute(cguidinterfacecount),
            ::std::mem::transmute(mszcards),
            ::std::mem::transmute(pcchcards),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardListCardsW(
    hcontext: usize,
    pbatr: *const u8,
    rgquidinterfaces: *const ::windows::runtime::GUID,
    cguidinterfacecount: u32,
    mszcards: super::super::Foundation::PWSTR,
    pcchcards: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardListCardsW(
                hcontext: usize,
                pbatr: *const u8,
                rgquidinterfaces: *const ::windows::runtime::GUID,
                cguidinterfacecount: u32,
                mszcards: super::super::Foundation::PWSTR,
                pcchcards: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(SCardListCardsW(
            ::std::mem::transmute(hcontext),
            ::std::mem::transmute(pbatr),
            ::std::mem::transmute(rgquidinterfaces),
            ::std::mem::transmute(cguidinterfacecount),
            ::std::mem::transmute(mszcards),
            ::std::mem::transmute(pcchcards),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardListInterfacesA<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hcontext: usize,
    szcard: Param1,
    pguidinterfaces: *mut ::windows::runtime::GUID,
    pcguidinterfaces: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardListInterfacesA(
                hcontext: usize,
                szcard: super::super::Foundation::PSTR,
                pguidinterfaces: *mut ::windows::runtime::GUID,
                pcguidinterfaces: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(SCardListInterfacesA(
            ::std::mem::transmute(hcontext),
            szcard.into_param().abi(),
            ::std::mem::transmute(pguidinterfaces),
            ::std::mem::transmute(pcguidinterfaces),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardListInterfacesW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hcontext: usize,
    szcard: Param1,
    pguidinterfaces: *mut ::windows::runtime::GUID,
    pcguidinterfaces: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardListInterfacesW(
                hcontext: usize,
                szcard: super::super::Foundation::PWSTR,
                pguidinterfaces: *mut ::windows::runtime::GUID,
                pcguidinterfaces: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(SCardListInterfacesW(
            ::std::mem::transmute(hcontext),
            szcard.into_param().abi(),
            ::std::mem::transmute(pguidinterfaces),
            ::std::mem::transmute(pcguidinterfaces),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardListReaderGroupsA(
    hcontext: usize,
    mszgroups: super::super::Foundation::PSTR,
    pcchgroups: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardListReaderGroupsA(
                hcontext: usize,
                mszgroups: super::super::Foundation::PSTR,
                pcchgroups: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(SCardListReaderGroupsA(
            ::std::mem::transmute(hcontext),
            ::std::mem::transmute(mszgroups),
            ::std::mem::transmute(pcchgroups),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardListReaderGroupsW(
    hcontext: usize,
    mszgroups: super::super::Foundation::PWSTR,
    pcchgroups: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardListReaderGroupsW(
                hcontext: usize,
                mszgroups: super::super::Foundation::PWSTR,
                pcchgroups: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(SCardListReaderGroupsW(
            ::std::mem::transmute(hcontext),
            ::std::mem::transmute(mszgroups),
            ::std::mem::transmute(pcchgroups),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardListReadersA<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hcontext: usize,
    mszgroups: Param1,
    mszreaders: super::super::Foundation::PSTR,
    pcchreaders: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardListReadersA(
                hcontext: usize,
                mszgroups: super::super::Foundation::PSTR,
                mszreaders: super::super::Foundation::PSTR,
                pcchreaders: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(SCardListReadersA(
            ::std::mem::transmute(hcontext),
            mszgroups.into_param().abi(),
            ::std::mem::transmute(mszreaders),
            ::std::mem::transmute(pcchreaders),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardListReadersW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hcontext: usize,
    mszgroups: Param1,
    mszreaders: super::super::Foundation::PWSTR,
    pcchreaders: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardListReadersW(
                hcontext: usize,
                mszgroups: super::super::Foundation::PWSTR,
                mszreaders: super::super::Foundation::PWSTR,
                pcchreaders: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(SCardListReadersW(
            ::std::mem::transmute(hcontext),
            mszgroups.into_param().abi(),
            ::std::mem::transmute(mszreaders),
            ::std::mem::transmute(pcchreaders),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardListReadersWithDeviceInstanceIdA<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hcontext: usize,
    szdeviceinstanceid: Param1,
    mszreaders: super::super::Foundation::PSTR,
    pcchreaders: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardListReadersWithDeviceInstanceIdA(
                hcontext: usize,
                szdeviceinstanceid: super::super::Foundation::PSTR,
                mszreaders: super::super::Foundation::PSTR,
                pcchreaders: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(SCardListReadersWithDeviceInstanceIdA(
            ::std::mem::transmute(hcontext),
            szdeviceinstanceid.into_param().abi(),
            ::std::mem::transmute(mszreaders),
            ::std::mem::transmute(pcchreaders),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardListReadersWithDeviceInstanceIdW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hcontext: usize,
    szdeviceinstanceid: Param1,
    mszreaders: super::super::Foundation::PWSTR,
    pcchreaders: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardListReadersWithDeviceInstanceIdW(
                hcontext: usize,
                szdeviceinstanceid: super::super::Foundation::PWSTR,
                mszreaders: super::super::Foundation::PWSTR,
                pcchreaders: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(SCardListReadersWithDeviceInstanceIdW(
            ::std::mem::transmute(hcontext),
            szdeviceinstanceid.into_param().abi(),
            ::std::mem::transmute(mszreaders),
            ::std::mem::transmute(pcchreaders),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardLocateCardsA<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hcontext: usize,
    mszcards: Param1,
    rgreaderstates: *mut SCARD_READERSTATEA,
    creaders: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardLocateCardsA(
                hcontext: usize,
                mszcards: super::super::Foundation::PSTR,
                rgreaderstates: *mut SCARD_READERSTATEA,
                creaders: u32,
            ) -> i32;
        }
        ::std::mem::transmute(SCardLocateCardsA(
            ::std::mem::transmute(hcontext),
            mszcards.into_param().abi(),
            ::std::mem::transmute(rgreaderstates),
            ::std::mem::transmute(creaders),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardLocateCardsByATRA(
    hcontext: usize,
    rgatrmasks: *const SCARD_ATRMASK,
    catrs: u32,
    rgreaderstates: *mut SCARD_READERSTATEA,
    creaders: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardLocateCardsByATRA(
                hcontext: usize,
                rgatrmasks: *const SCARD_ATRMASK,
                catrs: u32,
                rgreaderstates: *mut SCARD_READERSTATEA,
                creaders: u32,
            ) -> i32;
        }
        ::std::mem::transmute(SCardLocateCardsByATRA(
            ::std::mem::transmute(hcontext),
            ::std::mem::transmute(rgatrmasks),
            ::std::mem::transmute(catrs),
            ::std::mem::transmute(rgreaderstates),
            ::std::mem::transmute(creaders),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardLocateCardsByATRW(
    hcontext: usize,
    rgatrmasks: *const SCARD_ATRMASK,
    catrs: u32,
    rgreaderstates: *mut SCARD_READERSTATEW,
    creaders: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardLocateCardsByATRW(
                hcontext: usize,
                rgatrmasks: *const SCARD_ATRMASK,
                catrs: u32,
                rgreaderstates: *mut SCARD_READERSTATEW,
                creaders: u32,
            ) -> i32;
        }
        ::std::mem::transmute(SCardLocateCardsByATRW(
            ::std::mem::transmute(hcontext),
            ::std::mem::transmute(rgatrmasks),
            ::std::mem::transmute(catrs),
            ::std::mem::transmute(rgreaderstates),
            ::std::mem::transmute(creaders),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardLocateCardsW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hcontext: usize,
    mszcards: Param1,
    rgreaderstates: *mut SCARD_READERSTATEW,
    creaders: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardLocateCardsW(
                hcontext: usize,
                mszcards: super::super::Foundation::PWSTR,
                rgreaderstates: *mut SCARD_READERSTATEW,
                creaders: u32,
            ) -> i32;
        }
        ::std::mem::transmute(SCardLocateCardsW(
            ::std::mem::transmute(hcontext),
            mszcards.into_param().abi(),
            ::std::mem::transmute(rgreaderstates),
            ::std::mem::transmute(creaders),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardReadCacheA<
    'a,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hcontext: usize,
    cardidentifier: *const ::windows::runtime::GUID,
    freshnesscounter: u32,
    lookupname: Param3,
    data: *mut u8,
    datalen: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardReadCacheA(
                hcontext: usize,
                cardidentifier: *const ::windows::runtime::GUID,
                freshnesscounter: u32,
                lookupname: super::super::Foundation::PSTR,
                data: *mut u8,
                datalen: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(SCardReadCacheA(
            ::std::mem::transmute(hcontext),
            ::std::mem::transmute(cardidentifier),
            ::std::mem::transmute(freshnesscounter),
            lookupname.into_param().abi(),
            ::std::mem::transmute(data),
            ::std::mem::transmute(datalen),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardReadCacheW<
    'a,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hcontext: usize,
    cardidentifier: *const ::windows::runtime::GUID,
    freshnesscounter: u32,
    lookupname: Param3,
    data: *mut u8,
    datalen: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardReadCacheW(
                hcontext: usize,
                cardidentifier: *const ::windows::runtime::GUID,
                freshnesscounter: u32,
                lookupname: super::super::Foundation::PWSTR,
                data: *mut u8,
                datalen: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(SCardReadCacheW(
            ::std::mem::transmute(hcontext),
            ::std::mem::transmute(cardidentifier),
            ::std::mem::transmute(freshnesscounter),
            lookupname.into_param().abi(),
            ::std::mem::transmute(data),
            ::std::mem::transmute(datalen),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SCardReconnect(
    hcard: usize,
    dwsharemode: u32,
    dwpreferredprotocols: u32,
    dwinitialization: u32,
    pdwactiveprotocol: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardReconnect(
                hcard: usize,
                dwsharemode: u32,
                dwpreferredprotocols: u32,
                dwinitialization: u32,
                pdwactiveprotocol: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(SCardReconnect(
            ::std::mem::transmute(hcard),
            ::std::mem::transmute(dwsharemode),
            ::std::mem::transmute(dwpreferredprotocols),
            ::std::mem::transmute(dwinitialization),
            ::std::mem::transmute(pdwactiveprotocol),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SCardReleaseContext(hcontext: usize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardReleaseContext(hcontext: usize) -> i32;
        }
        ::std::mem::transmute(SCardReleaseContext(::std::mem::transmute(hcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SCardReleaseStartedEvent() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardReleaseStartedEvent();
        }
        ::std::mem::transmute(SCardReleaseStartedEvent())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardRemoveReaderFromGroupA<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hcontext: usize,
    szreadername: Param1,
    szgroupname: Param2,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardRemoveReaderFromGroupA(
                hcontext: usize,
                szreadername: super::super::Foundation::PSTR,
                szgroupname: super::super::Foundation::PSTR,
            ) -> i32;
        }
        ::std::mem::transmute(SCardRemoveReaderFromGroupA(
            ::std::mem::transmute(hcontext),
            szreadername.into_param().abi(),
            szgroupname.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardRemoveReaderFromGroupW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hcontext: usize,
    szreadername: Param1,
    szgroupname: Param2,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardRemoveReaderFromGroupW(
                hcontext: usize,
                szreadername: super::super::Foundation::PWSTR,
                szgroupname: super::super::Foundation::PWSTR,
            ) -> i32;
        }
        ::std::mem::transmute(SCardRemoveReaderFromGroupW(
            ::std::mem::transmute(hcontext),
            szreadername.into_param().abi(),
            szgroupname.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SCardSetAttrib(
    hcard: usize,
    dwattrid: u32,
    pbattr: *const u8,
    cbattrlen: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardSetAttrib(
                hcard: usize,
                dwattrid: u32,
                pbattr: *const u8,
                cbattrlen: u32,
            ) -> i32;
        }
        ::std::mem::transmute(SCardSetAttrib(
            ::std::mem::transmute(hcard),
            ::std::mem::transmute(dwattrid),
            ::std::mem::transmute(pbattr),
            ::std::mem::transmute(cbattrlen),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardSetCardTypeProviderNameA<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hcontext: usize,
    szcardname: Param1,
    dwproviderid: u32,
    szprovider: Param3,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardSetCardTypeProviderNameA(
                hcontext: usize,
                szcardname: super::super::Foundation::PSTR,
                dwproviderid: u32,
                szprovider: super::super::Foundation::PSTR,
            ) -> i32;
        }
        ::std::mem::transmute(SCardSetCardTypeProviderNameA(
            ::std::mem::transmute(hcontext),
            szcardname.into_param().abi(),
            ::std::mem::transmute(dwproviderid),
            szprovider.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardSetCardTypeProviderNameW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hcontext: usize,
    szcardname: Param1,
    dwproviderid: u32,
    szprovider: Param3,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardSetCardTypeProviderNameW(
                hcontext: usize,
                szcardname: super::super::Foundation::PWSTR,
                dwproviderid: u32,
                szprovider: super::super::Foundation::PWSTR,
            ) -> i32;
        }
        ::std::mem::transmute(SCardSetCardTypeProviderNameW(
            ::std::mem::transmute(hcontext),
            szcardname.into_param().abi(),
            ::std::mem::transmute(dwproviderid),
            szprovider.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SCardState(
    hcard: usize,
    pdwstate: *mut u32,
    pdwprotocol: *mut u32,
    pbatr: *mut u8,
    pcbatrlen: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardState(
                hcard: usize,
                pdwstate: *mut u32,
                pdwprotocol: *mut u32,
                pbatr: *mut u8,
                pcbatrlen: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(SCardState(
            ::std::mem::transmute(hcard),
            ::std::mem::transmute(pdwstate),
            ::std::mem::transmute(pdwprotocol),
            ::std::mem::transmute(pbatr),
            ::std::mem::transmute(pcbatrlen),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardStatusA(
    hcard: usize,
    mszreadernames: super::super::Foundation::PSTR,
    pcchreaderlen: *mut u32,
    pdwstate: *mut u32,
    pdwprotocol: *mut u32,
    pbatr: *mut u8,
    pcbatrlen: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardStatusA(
                hcard: usize,
                mszreadernames: super::super::Foundation::PSTR,
                pcchreaderlen: *mut u32,
                pdwstate: *mut u32,
                pdwprotocol: *mut u32,
                pbatr: *mut u8,
                pcbatrlen: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(SCardStatusA(
            ::std::mem::transmute(hcard),
            ::std::mem::transmute(mszreadernames),
            ::std::mem::transmute(pcchreaderlen),
            ::std::mem::transmute(pdwstate),
            ::std::mem::transmute(pdwprotocol),
            ::std::mem::transmute(pbatr),
            ::std::mem::transmute(pcbatrlen),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardStatusW(
    hcard: usize,
    mszreadernames: super::super::Foundation::PWSTR,
    pcchreaderlen: *mut u32,
    pdwstate: *mut u32,
    pdwprotocol: *mut u32,
    pbatr: *mut u8,
    pcbatrlen: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardStatusW(
                hcard: usize,
                mszreadernames: super::super::Foundation::PWSTR,
                pcchreaderlen: *mut u32,
                pdwstate: *mut u32,
                pdwprotocol: *mut u32,
                pbatr: *mut u8,
                pcbatrlen: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(SCardStatusW(
            ::std::mem::transmute(hcard),
            ::std::mem::transmute(mszreadernames),
            ::std::mem::transmute(pcchreaderlen),
            ::std::mem::transmute(pdwstate),
            ::std::mem::transmute(pdwprotocol),
            ::std::mem::transmute(pbatr),
            ::std::mem::transmute(pcbatrlen),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SCardTransmit(
    hcard: usize,
    piosendpci: *const SCARD_IO_REQUEST,
    pbsendbuffer: *const u8,
    cbsendlength: u32,
    piorecvpci: *mut SCARD_IO_REQUEST,
    pbrecvbuffer: *mut u8,
    pcbrecvlength: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardTransmit(
                hcard: usize,
                piosendpci: *const SCARD_IO_REQUEST,
                pbsendbuffer: *const u8,
                cbsendlength: u32,
                piorecvpci: *mut SCARD_IO_REQUEST,
                pbrecvbuffer: *mut u8,
                pcbrecvlength: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(SCardTransmit(
            ::std::mem::transmute(hcard),
            ::std::mem::transmute(piosendpci),
            ::std::mem::transmute(pbsendbuffer),
            ::std::mem::transmute(cbsendlength),
            ::std::mem::transmute(piorecvpci),
            ::std::mem::transmute(pbrecvbuffer),
            ::std::mem::transmute(pcbrecvlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn SCardUIDlgSelectCardA(param0: *mut OPENCARDNAME_EXA) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardUIDlgSelectCardA(
                param0: *mut ::std::mem::ManuallyDrop<OPENCARDNAME_EXA>,
            ) -> i32;
        }
        ::std::mem::transmute(SCardUIDlgSelectCardA(::std::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn SCardUIDlgSelectCardW(param0: *mut OPENCARDNAME_EXW) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardUIDlgSelectCardW(
                param0: *mut ::std::mem::ManuallyDrop<OPENCARDNAME_EXW>,
            ) -> i32;
        }
        ::std::mem::transmute(SCardUIDlgSelectCardW(::std::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardWriteCacheA<
    'a,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hcontext: usize,
    cardidentifier: *const ::windows::runtime::GUID,
    freshnesscounter: u32,
    lookupname: Param3,
    data: *const u8,
    datalen: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardWriteCacheA(
                hcontext: usize,
                cardidentifier: *const ::windows::runtime::GUID,
                freshnesscounter: u32,
                lookupname: super::super::Foundation::PSTR,
                data: *const u8,
                datalen: u32,
            ) -> i32;
        }
        ::std::mem::transmute(SCardWriteCacheA(
            ::std::mem::transmute(hcontext),
            ::std::mem::transmute(cardidentifier),
            ::std::mem::transmute(freshnesscounter),
            lookupname.into_param().abi(),
            ::std::mem::transmute(data),
            ::std::mem::transmute(datalen),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardWriteCacheW<
    'a,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hcontext: usize,
    cardidentifier: *const ::windows::runtime::GUID,
    freshnesscounter: u32,
    lookupname: Param3,
    data: *const u8,
    datalen: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SCardWriteCacheW(
                hcontext: usize,
                cardidentifier: *const ::windows::runtime::GUID,
                freshnesscounter: u32,
                lookupname: super::super::Foundation::PWSTR,
                data: *const u8,
                datalen: u32,
            ) -> i32;
        }
        ::std::mem::transmute(SCardWriteCacheW(
            ::std::mem::transmute(hcontext),
            ::std::mem::transmute(cardidentifier),
            ::std::mem::transmute(freshnesscounter),
            lookupname.into_param().abi(),
            ::std::mem::transmute(data),
            ::std::mem::transmute(datalen),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const SECPKG_ALT_ATTR: u32 = 2147483648u32;
pub const SECPKG_ATTR_C_FULL_IDENT_TOKEN: u32 = 2147483781u32;
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_ACCESS_DENIED: super::super::Foundation::NTSTATUS =
    super::super::Foundation::NTSTATUS(-1073741790i32 as _);
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_ACCOUNT_DISABLED: super::super::Foundation::NTSTATUS =
    super::super::Foundation::NTSTATUS(-1073741710i32 as _);
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_ACCOUNT_EXPIRED: super::super::Foundation::NTSTATUS =
    super::super::Foundation::NTSTATUS(-1073741421i32 as _);
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_ACCOUNT_LOCKED_OUT: super::super::Foundation::NTSTATUS =
    super::super::Foundation::NTSTATUS(-1073741260i32 as _);
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_ACCOUNT_RESTRICTION: super::super::Foundation::NTSTATUS =
    super::super::Foundation::NTSTATUS(-1073741714i32 as _);
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_AUTHENTICATION_FIREWALL_FAILED: super::super::Foundation::NTSTATUS =
    super::super::Foundation::NTSTATUS(-1073740781i32 as _);
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_DOWNGRADE_DETECTED: super::super::Foundation::NTSTATUS =
    super::super::Foundation::NTSTATUS(-1073740920i32 as _);
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_LOGON_FAILURE: super::super::Foundation::NTSTATUS =
    super::super::Foundation::NTSTATUS(-1073741715i32 as _);
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_LOGON_TYPE_NOT_GRANTED: super::super::Foundation::NTSTATUS =
    super::super::Foundation::NTSTATUS(-1073741477i32 as _);
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_NO_SUCH_LOGON_SESSION: super::super::Foundation::NTSTATUS =
    super::super::Foundation::NTSTATUS(-1073741729i32 as _);
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_NO_SUCH_USER: super::super::Foundation::NTSTATUS =
    super::super::Foundation::NTSTATUS(-1073741724i32 as _);
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_PASSWORD_EXPIRED: super::super::Foundation::NTSTATUS =
    super::super::Foundation::NTSTATUS(-1073741711i32 as _);
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_PASSWORD_MUST_CHANGE: super::super::Foundation::NTSTATUS =
    super::super::Foundation::NTSTATUS(-1073741276i32 as _);
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_WRONG_PASSWORD: super::super::Foundation::NTSTATUS =
    super::super::Foundation::NTSTATUS(-1073741718i32 as _);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SecHandle {
    pub dwLower: usize,
    pub dwUpper: usize,
}
impl SecHandle {}
impl ::std::default::Default for SecHandle {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SecHandle {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SecHandle")
            .field("dwLower", &self.dwLower)
            .field("dwUpper", &self.dwUpper)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SecHandle {
    fn eq(&self, other: &Self) -> bool {
        self.dwLower == other.dwLower && self.dwUpper == other.dwUpper
    }
}
impl ::std::cmp::Eq for SecHandle {}
unsafe impl ::windows::runtime::Abi for SecHandle {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SecPkgContext_ClientCreds {
    pub AuthBufferLen: u32,
    pub AuthBuffer: *mut u8,
}
impl SecPkgContext_ClientCreds {}
impl ::std::default::Default for SecPkgContext_ClientCreds {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SecPkgContext_ClientCreds {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SecPkgContext_ClientCreds")
            .field("AuthBufferLen", &self.AuthBufferLen)
            .field("AuthBuffer", &self.AuthBuffer)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SecPkgContext_ClientCreds {
    fn eq(&self, other: &Self) -> bool {
        self.AuthBufferLen == other.AuthBufferLen && self.AuthBuffer == other.AuthBuffer
    }
}
impl ::std::cmp::Eq for SecPkgContext_ClientCreds {}
unsafe impl ::windows::runtime::Abi for SecPkgContext_ClientCreds {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct USERNAME_TARGET_CREDENTIAL_INFO {
    pub UserName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl USERNAME_TARGET_CREDENTIAL_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for USERNAME_TARGET_CREDENTIAL_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for USERNAME_TARGET_CREDENTIAL_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("USERNAME_TARGET_CREDENTIAL_INFO")
            .field("UserName", &self.UserName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for USERNAME_TARGET_CREDENTIAL_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.UserName == other.UserName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for USERNAME_TARGET_CREDENTIAL_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for USERNAME_TARGET_CREDENTIAL_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
