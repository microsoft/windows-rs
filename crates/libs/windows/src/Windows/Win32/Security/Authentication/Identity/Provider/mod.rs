pub const CONNECTING: ACCOUNT_STATE = 1i32;
pub const CONNECT_COMPLETED: ACCOUNT_STATE = 2i32;
pub const IDENTITIES_ALL: IDENTITY_TYPE = 0i32;
pub const IDENTITIES_ME_ONLY: IDENTITY_TYPE = 1i32;
pub const IDENTITY_ASSOCIATED: IdentityUpdateEvent = 1i32;
pub const IDENTITY_CONNECTED: IdentityUpdateEvent = 64i32;
pub const IDENTITY_CREATED: IdentityUpdateEvent = 4i32;
pub const IDENTITY_DELETED: IdentityUpdateEvent = 16i32;
pub const IDENTITY_DISASSOCIATED: IdentityUpdateEvent = 2i32;
pub const IDENTITY_DISCONNECTED: IdentityUpdateEvent = 128i32;
pub const IDENTITY_IMPORTED: IdentityUpdateEvent = 8i32;
pub const IDENTITY_KEYWORD_ASSOCIATED: windows_core::PCWSTR = windows_core::w!("associated");
pub const IDENTITY_KEYWORD_CONNECTED: windows_core::PCWSTR = windows_core::w!("connected");
pub const IDENTITY_KEYWORD_HOMEGROUP: windows_core::PCWSTR = windows_core::w!("homegroup");
pub const IDENTITY_KEYWORD_LOCAL: windows_core::PCWSTR = windows_core::w!("local");
pub const IDENTITY_PROPCHANGED: IdentityUpdateEvent = 32i32;
pub const IDENTITY_URL_ACCOUNT_SETTINGS: IDENTITY_URL = 4i32;
pub const IDENTITY_URL_CHANGE_PASSWORD_WIZARD: IDENTITY_URL = 2i32;
pub const IDENTITY_URL_CONNECT_WIZARD: IDENTITY_URL = 6i32;
pub const IDENTITY_URL_CREATE_ACCOUNT_WIZARD: IDENTITY_URL = 0i32;
pub const IDENTITY_URL_IFEXISTS_WIZARD: IDENTITY_URL = 3i32;
pub const IDENTITY_URL_RESTORE_WIZARD: IDENTITY_URL = 5i32;
pub const IDENTITY_URL_SIGN_IN_WIZARD: IDENTITY_URL = 1i32;
pub const NOT_CONNECTED: ACCOUNT_STATE = 0i32;
pub const OID_OAssociatedIdentityProviderObject: windows_core::GUID = windows_core::GUID::from_u128(0x98c5a3dd_db68_4f1a_8d2b_9079cdfeaf61);
pub const STR_COMPLETE_ACCOUNT: windows_core::PCWSTR = windows_core::w!("CompleteAccount");
pub const STR_MODERN_SETTINGS_ADD_USER: windows_core::PCWSTR = windows_core::w!("ModernSettingsAddUser");
pub const STR_NTH_USER_FIRST_AUTH: windows_core::PCWSTR = windows_core::w!("NthUserFirstAuth");
pub const STR_OUT_OF_BOX_EXPERIENCE: windows_core::PCWSTR = windows_core::w!("OutOfBoxExperience");
pub const STR_OUT_OF_BOX_UPGRADE_EXPERIENCE: windows_core::PCWSTR = windows_core::w!("OutOfBoxUpgradeExperience");
pub const STR_PROPERTY_STORE: windows_core::PCWSTR = windows_core::w!("PropertyStore");
pub const STR_USER_NAME: windows_core::PCWSTR = windows_core::w!("Username");
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ACCOUNT_STATE(pub i32);
impl windows_core::TypeKind for ACCOUNT_STATE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IDENTITY_TYPE(pub i32);
impl windows_core::TypeKind for IDENTITY_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IDENTITY_URL(pub i32);
impl windows_core::TypeKind for IDENTITY_URL {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IdentityUpdateEvent(pub i32);
impl windows_core::TypeKind for IdentityUpdateEvent {
    type TypeKind = windows_core::CopyType;
}
pub const CIdentityProfileHandler: windows_core::GUID = windows_core::GUID::from_u128(0xecf5bf46_e3b6_449a_b56b_43f58f867814);
pub const CoClassIdentityStore: windows_core::GUID = windows_core::GUID::from_u128(0x30d49246_d217_465f_b00b_ac9ddd652eb7);
