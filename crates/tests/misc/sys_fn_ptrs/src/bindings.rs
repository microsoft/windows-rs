#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

pub type BOOL = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GUID {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}
impl GUID {
    pub const fn from_u128(uuid: u128) -> Self {
        Self {
            data1: (uuid >> 96) as u32,
            data2: (uuid >> 80 & 0xffff) as u16,
            data3: (uuid >> 64 & 0xffff) as u16,
            data4: (uuid as u64).to_be_bytes(),
        }
    }
}
pub type HRESULT = i32;
pub type PCSTR = *const u8;
pub type PCWSTR = *const u16;
pub mod Windows {
    pub mod Win32 {
        pub mod Foundation {
            pub type FreeLibrary =
                unsafe extern "system" fn(hlibmodule: HMODULE) -> super::super::super::BOOL;
            windows_link::link!("kernel32.dll" "system" fn FreeLibrary(hlibmodule : HMODULE) -> super::super::super::BOOL);
            pub type FARPROC = Option<unsafe extern "system" fn() -> isize>;
            pub type HANDLE = *mut core::ffi::c_void;
            pub type HINSTANCE = *mut core::ffi::c_void;
            pub type HMODULE = *mut core::ffi::c_void;
            pub type HWND = *mut core::ffi::c_void;
        }
        pub mod Networking {
            pub mod WindowsWebServices {
                pub type WebAuthNAuthenticatorGetAssertion = unsafe extern "system" fn(hwnd : super::super::Foundation:: HWND, pwszrpid : super::super::super::super::PCWSTR, pwebauthnclientdata : *const WEBAUTHN_CLIENT_DATA, pwebauthngetassertionoptions : *const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS, ppwebauthnassertion : *mut *mut WEBAUTHN_ASSERTION) -> super::super::super::super::HRESULT ;
                windows_link::link!("webauthn.dll" "system" fn WebAuthNAuthenticatorGetAssertion(hwnd : super::super::Foundation:: HWND, pwszrpid : super::super::super::super::PCWSTR, pwebauthnclientdata : *const WEBAUTHN_CLIENT_DATA, pwebauthngetassertionoptions : *const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS, ppwebauthnassertion : *mut *mut WEBAUTHN_ASSERTION) -> super::super::super::super::HRESULT);
                pub type WebAuthNAuthenticatorMakeCredential = unsafe extern "system" fn(hwnd : super::super::Foundation:: HWND, prpinformation : *const WEBAUTHN_RP_ENTITY_INFORMATION, puserinformation : *const WEBAUTHN_USER_ENTITY_INFORMATION, ppubkeycredparams : *const WEBAUTHN_COSE_CREDENTIAL_PARAMETERS, pwebauthnclientdata : *const WEBAUTHN_CLIENT_DATA, pwebauthnmakecredentialoptions : *const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS, ppwebauthncredentialattestation : *mut *mut WEBAUTHN_CREDENTIAL_ATTESTATION) -> super::super::super::super::HRESULT ;
                windows_link::link!("webauthn.dll" "system" fn WebAuthNAuthenticatorMakeCredential(hwnd : super::super::Foundation:: HWND, prpinformation : *const WEBAUTHN_RP_ENTITY_INFORMATION, puserinformation : *const WEBAUTHN_USER_ENTITY_INFORMATION, ppubkeycredparams : *const WEBAUTHN_COSE_CREDENTIAL_PARAMETERS, pwebauthnclientdata : *const WEBAUTHN_CLIENT_DATA, pwebauthnmakecredentialoptions : *const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS, ppwebauthncredentialattestation : *mut *mut WEBAUTHN_CREDENTIAL_ATTESTATION) -> super::super::super::super::HRESULT);
                #[repr(C)]
                #[derive(Clone, Copy)]
                pub struct CTAPCBOR_HYBRID_STORAGE_LINKED_DATA {
                    pub dwVersion: u32,
                    pub cbContactId: u32,
                    pub pbContactId: *mut u8,
                    pub cbLinkId: u32,
                    pub pbLinkId: *mut u8,
                    pub cbLinkSecret: u32,
                    pub pbLinkSecret: *mut u8,
                    pub cbPublicKey: u32,
                    pub pbPublicKey: *mut u8,
                    pub pwszAuthenticatorName: super::super::super::super::PCWSTR,
                    pub wEncodedTunnelServerDomain: u16,
                }
                impl Default for CTAPCBOR_HYBRID_STORAGE_LINKED_DATA {
                    fn default() -> Self {
                        unsafe { core::mem::zeroed() }
                    }
                }
                #[repr(C)]
                #[derive(Clone, Copy)]
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
                    pub pHmacSecret: *mut WEBAUTHN_HMAC_SECRET_SALT,
                    pub dwUsedTransport: u32,
                    pub cbUnsignedExtensionOutputs: u32,
                    pub pbUnsignedExtensionOutputs: *mut u8,
                }
                impl Default for WEBAUTHN_ASSERTION {
                    fn default() -> Self {
                        unsafe { core::mem::zeroed() }
                    }
                }
                #[repr(C)]
                #[derive(Clone, Copy)]
                pub struct WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS {
                    pub dwVersion: u32,
                    pub dwTimeoutMilliseconds: u32,
                    pub CredentialList: WEBAUTHN_CREDENTIALS,
                    pub Extensions: WEBAUTHN_EXTENSIONS,
                    pub dwAuthenticatorAttachment: u32,
                    pub dwUserVerificationRequirement: u32,
                    pub dwFlags: u32,
                    pub pwszU2fAppId: super::super::super::super::PCWSTR,
                    pub pbU2fAppId: *mut super::super::super::super::BOOL,
                    pub pCancellationId: *mut super::super::super::super::GUID,
                    pub pAllowCredentialList: *mut WEBAUTHN_CREDENTIAL_LIST,
                    pub dwCredLargeBlobOperation: u32,
                    pub cbCredLargeBlob: u32,
                    pub pbCredLargeBlob: *mut u8,
                    pub pHmacSecretSaltValues: *mut WEBAUTHN_HMAC_SECRET_SALT_VALUES,
                    pub bBrowserInPrivateMode: super::super::super::super::BOOL,
                    pub pLinkedDevice: *mut CTAPCBOR_HYBRID_STORAGE_LINKED_DATA,
                    pub bAutoFill: super::super::super::super::BOOL,
                    pub cbJsonExt: u32,
                    pub pbJsonExt: *mut u8,
                }
                impl Default for WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS {
                    fn default() -> Self {
                        unsafe { core::mem::zeroed() }
                    }
                }
                #[repr(C)]
                #[derive(Clone, Copy)]
                pub struct WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS {
                    pub dwVersion: u32,
                    pub dwTimeoutMilliseconds: u32,
                    pub CredentialList: WEBAUTHN_CREDENTIALS,
                    pub Extensions: WEBAUTHN_EXTENSIONS,
                    pub dwAuthenticatorAttachment: u32,
                    pub bRequireResidentKey: super::super::super::super::BOOL,
                    pub dwUserVerificationRequirement: u32,
                    pub dwAttestationConveyancePreference: u32,
                    pub dwFlags: u32,
                    pub pCancellationId: *mut super::super::super::super::GUID,
                    pub pExcludeCredentialList: *mut WEBAUTHN_CREDENTIAL_LIST,
                    pub dwEnterpriseAttestation: u32,
                    pub dwLargeBlobSupport: u32,
                    pub bPreferResidentKey: super::super::super::super::BOOL,
                    pub bBrowserInPrivateMode: super::super::super::super::BOOL,
                    pub bEnablePrf: super::super::super::super::BOOL,
                    pub pLinkedDevice: *mut CTAPCBOR_HYBRID_STORAGE_LINKED_DATA,
                    pub cbJsonExt: u32,
                    pub pbJsonExt: *mut u8,
                }
                impl Default for WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS {
                    fn default() -> Self {
                        unsafe { core::mem::zeroed() }
                    }
                }
                #[repr(C)]
                #[derive(Clone, Copy)]
                pub struct WEBAUTHN_CLIENT_DATA {
                    pub dwVersion: u32,
                    pub cbClientDataJSON: u32,
                    pub pbClientDataJSON: *mut u8,
                    pub pwszHashAlgId: super::super::super::super::PCWSTR,
                }
                impl Default for WEBAUTHN_CLIENT_DATA {
                    fn default() -> Self {
                        unsafe { core::mem::zeroed() }
                    }
                }
                #[repr(C)]
                #[derive(Clone, Copy)]
                pub struct WEBAUTHN_COSE_CREDENTIAL_PARAMETER {
                    pub dwVersion: u32,
                    pub pwszCredentialType: super::super::super::super::PCWSTR,
                    pub lAlg: i32,
                }
                impl Default for WEBAUTHN_COSE_CREDENTIAL_PARAMETER {
                    fn default() -> Self {
                        unsafe { core::mem::zeroed() }
                    }
                }
                #[repr(C)]
                #[derive(Clone, Copy)]
                pub struct WEBAUTHN_COSE_CREDENTIAL_PARAMETERS {
                    pub cCredentialParameters: u32,
                    pub pCredentialParameters: *mut WEBAUTHN_COSE_CREDENTIAL_PARAMETER,
                }
                impl Default for WEBAUTHN_COSE_CREDENTIAL_PARAMETERS {
                    fn default() -> Self {
                        unsafe { core::mem::zeroed() }
                    }
                }
                #[repr(C)]
                #[derive(Clone, Copy)]
                pub struct WEBAUTHN_CREDENTIAL {
                    pub dwVersion: u32,
                    pub cbId: u32,
                    pub pbId: *mut u8,
                    pub pwszCredentialType: super::super::super::super::PCWSTR,
                }
                impl Default for WEBAUTHN_CREDENTIAL {
                    fn default() -> Self {
                        unsafe { core::mem::zeroed() }
                    }
                }
                #[repr(C)]
                #[derive(Clone, Copy)]
                pub struct WEBAUTHN_CREDENTIALS {
                    pub cCredentials: u32,
                    pub pCredentials: *mut WEBAUTHN_CREDENTIAL,
                }
                impl Default for WEBAUTHN_CREDENTIALS {
                    fn default() -> Self {
                        unsafe { core::mem::zeroed() }
                    }
                }
                #[repr(C)]
                #[derive(Clone, Copy)]
                pub struct WEBAUTHN_CREDENTIAL_ATTESTATION {
                    pub dwVersion: u32,
                    pub pwszFormatType: super::super::super::super::PCWSTR,
                    pub cbAuthenticatorData: u32,
                    pub pbAuthenticatorData: *mut u8,
                    pub cbAttestation: u32,
                    pub pbAttestation: *mut u8,
                    pub dwAttestationDecodeType: u32,
                    pub pvAttestationDecode: *mut core::ffi::c_void,
                    pub cbAttestationObject: u32,
                    pub pbAttestationObject: *mut u8,
                    pub cbCredentialId: u32,
                    pub pbCredentialId: *mut u8,
                    pub Extensions: WEBAUTHN_EXTENSIONS,
                    pub dwUsedTransport: u32,
                    pub bEpAtt: super::super::super::super::BOOL,
                    pub bLargeBlobSupported: super::super::super::super::BOOL,
                    pub bResidentKey: super::super::super::super::BOOL,
                    pub bPrfEnabled: super::super::super::super::BOOL,
                    pub cbUnsignedExtensionOutputs: u32,
                    pub pbUnsignedExtensionOutputs: *mut u8,
                }
                impl Default for WEBAUTHN_CREDENTIAL_ATTESTATION {
                    fn default() -> Self {
                        unsafe { core::mem::zeroed() }
                    }
                }
                #[repr(C)]
                #[derive(Clone, Copy)]
                pub struct WEBAUTHN_CREDENTIAL_EX {
                    pub dwVersion: u32,
                    pub cbId: u32,
                    pub pbId: *mut u8,
                    pub pwszCredentialType: super::super::super::super::PCWSTR,
                    pub dwTransports: u32,
                }
                impl Default for WEBAUTHN_CREDENTIAL_EX {
                    fn default() -> Self {
                        unsafe { core::mem::zeroed() }
                    }
                }
                #[repr(C)]
                #[derive(Clone, Copy)]
                pub struct WEBAUTHN_CREDENTIAL_LIST {
                    pub cCredentials: u32,
                    pub ppCredentials: *mut *mut WEBAUTHN_CREDENTIAL_EX,
                }
                impl Default for WEBAUTHN_CREDENTIAL_LIST {
                    fn default() -> Self {
                        unsafe { core::mem::zeroed() }
                    }
                }
                #[repr(C)]
                #[derive(Clone, Copy)]
                pub struct WEBAUTHN_CRED_WITH_HMAC_SECRET_SALT {
                    pub cbCredID: u32,
                    pub pbCredID: *mut u8,
                    pub pHmacSecretSalt: *mut WEBAUTHN_HMAC_SECRET_SALT,
                }
                impl Default for WEBAUTHN_CRED_WITH_HMAC_SECRET_SALT {
                    fn default() -> Self {
                        unsafe { core::mem::zeroed() }
                    }
                }
                #[repr(C)]
                #[derive(Clone, Copy)]
                pub struct WEBAUTHN_EXTENSION {
                    pub pwszExtensionIdentifier: super::super::super::super::PCWSTR,
                    pub cbExtension: u32,
                    pub pvExtension: *mut core::ffi::c_void,
                }
                impl Default for WEBAUTHN_EXTENSION {
                    fn default() -> Self {
                        unsafe { core::mem::zeroed() }
                    }
                }
                #[repr(C)]
                #[derive(Clone, Copy)]
                pub struct WEBAUTHN_EXTENSIONS {
                    pub cExtensions: u32,
                    pub pExtensions: *mut WEBAUTHN_EXTENSION,
                }
                impl Default for WEBAUTHN_EXTENSIONS {
                    fn default() -> Self {
                        unsafe { core::mem::zeroed() }
                    }
                }
                #[repr(C)]
                #[derive(Clone, Copy)]
                pub struct WEBAUTHN_HMAC_SECRET_SALT {
                    pub cbFirst: u32,
                    pub pbFirst: *mut u8,
                    pub cbSecond: u32,
                    pub pbSecond: *mut u8,
                }
                impl Default for WEBAUTHN_HMAC_SECRET_SALT {
                    fn default() -> Self {
                        unsafe { core::mem::zeroed() }
                    }
                }
                #[repr(C)]
                #[derive(Clone, Copy)]
                pub struct WEBAUTHN_HMAC_SECRET_SALT_VALUES {
                    pub pGlobalHmacSalt: *mut WEBAUTHN_HMAC_SECRET_SALT,
                    pub cCredWithHmacSecretSaltList: u32,
                    pub pCredWithHmacSecretSaltList: *mut WEBAUTHN_CRED_WITH_HMAC_SECRET_SALT,
                }
                impl Default for WEBAUTHN_HMAC_SECRET_SALT_VALUES {
                    fn default() -> Self {
                        unsafe { core::mem::zeroed() }
                    }
                }
                #[repr(C)]
                #[derive(Clone, Copy)]
                pub struct WEBAUTHN_RP_ENTITY_INFORMATION {
                    pub dwVersion: u32,
                    pub pwszId: super::super::super::super::PCWSTR,
                    pub pwszName: super::super::super::super::PCWSTR,
                    pub pwszIcon: super::super::super::super::PCWSTR,
                }
                impl Default for WEBAUTHN_RP_ENTITY_INFORMATION {
                    fn default() -> Self {
                        unsafe { core::mem::zeroed() }
                    }
                }
                #[repr(C)]
                #[derive(Clone, Copy)]
                pub struct WEBAUTHN_USER_ENTITY_INFORMATION {
                    pub dwVersion: u32,
                    pub cbId: u32,
                    pub pbId: *mut u8,
                    pub pwszName: super::super::super::super::PCWSTR,
                    pub pwszIcon: super::super::super::super::PCWSTR,
                    pub pwszDisplayName: super::super::super::super::PCWSTR,
                }
                impl Default for WEBAUTHN_USER_ENTITY_INFORMATION {
                    fn default() -> Self {
                        unsafe { core::mem::zeroed() }
                    }
                }
            }
        }
        pub mod System {
            pub mod LibraryLoader {
                pub type GetProcAddress =
                    unsafe extern "system" fn(
                        hmodule: super::super::Foundation::HMODULE,
                        lpprocname: super::super::super::super::PCSTR,
                    )
                        -> super::super::Foundation::FARPROC;
                windows_link::link!("kernel32.dll" "system" fn GetProcAddress(hmodule : super::super::Foundation:: HMODULE, lpprocname : super::super::super::super::PCSTR) -> super::super::Foundation:: FARPROC);
                pub type LoadLibraryExA =
                    unsafe extern "system" fn(
                        lplibfilename: super::super::super::super::PCSTR,
                        hfile: super::super::Foundation::HANDLE,
                        dwflags: LOAD_LIBRARY_FLAGS,
                    )
                        -> super::super::Foundation::HMODULE;
                windows_link::link!("kernel32.dll" "system" fn LoadLibraryExA(lplibfilename : super::super::super::super::PCSTR, hfile : super::super::Foundation:: HANDLE, dwflags : LOAD_LIBRARY_FLAGS) -> super::super::Foundation:: HMODULE);
                pub type LOAD_LIBRARY_FLAGS = u32;
                pub const LOAD_LIBRARY_SEARCH_DEFAULT_DIRS: LOAD_LIBRARY_FLAGS = 4096u32;
            }
        }
    }
}
