#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckGamingPrivilegeSilently(privilegeid: u32, scope: ::windows_sys::core::HSTRING, policy: ::windows_sys::core::HSTRING, hasprivilege: *mut super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckGamingPrivilegeSilentlyForUser(user: ::windows_sys::core::IInspectable, privilegeid: u32, scope: ::windows_sys::core::HSTRING, policy: ::windows_sys::core::HSTRING, hasprivilege: *mut super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    pub fn CheckGamingPrivilegeWithUI(privilegeid: u32, scope: ::windows_sys::core::HSTRING, policy: ::windows_sys::core::HSTRING, friendlymessage: ::windows_sys::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn CheckGamingPrivilegeWithUIForUser(user: ::windows_sys::core::IInspectable, privilegeid: u32, scope: ::windows_sys::core::HSTRING, policy: ::windows_sys::core::HSTRING, friendlymessage: ::windows_sys::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn GetExpandedResourceExclusiveCpuCount(exclusivecpucount: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn GetGamingDeviceModelInformation(information: *mut GAMING_DEVICE_MODEL_INFORMATION) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HasExpandedResources(hasexpandedresources: *mut super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ProcessPendingGameUI(waitforcompletion: super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    pub fn ReleaseExclusiveCpuSets() -> ::windows_sys::core::HRESULT;
    pub fn ShowChangeFriendRelationshipUI(targetuserxuid: ::windows_sys::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn ShowChangeFriendRelationshipUIForUser(user: ::windows_sys::core::IInspectable, targetuserxuid: ::windows_sys::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn ShowCustomizeUserProfileUI(completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn ShowCustomizeUserProfileUIForUser(user: ::windows_sys::core::IInspectable, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn ShowFindFriendsUI(completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn ShowFindFriendsUIForUser(user: ::windows_sys::core::IInspectable, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn ShowGameInfoUI(titleid: u32, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn ShowGameInfoUIForUser(user: ::windows_sys::core::IInspectable, titleid: u32, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn ShowGameInviteUI(serviceconfigurationid: ::windows_sys::core::HSTRING, sessiontemplatename: ::windows_sys::core::HSTRING, sessionid: ::windows_sys::core::HSTRING, invitationdisplaytext: ::windows_sys::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn ShowGameInviteUIForUser(user: ::windows_sys::core::IInspectable, serviceconfigurationid: ::windows_sys::core::HSTRING, sessiontemplatename: ::windows_sys::core::HSTRING, sessionid: ::windows_sys::core::HSTRING, invitationdisplaytext: ::windows_sys::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn ShowGameInviteUIWithContext(serviceconfigurationid: ::windows_sys::core::HSTRING, sessiontemplatename: ::windows_sys::core::HSTRING, sessionid: ::windows_sys::core::HSTRING, invitationdisplaytext: ::windows_sys::core::HSTRING, customactivationcontext: ::windows_sys::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn ShowGameInviteUIWithContextForUser(user: ::windows_sys::core::IInspectable, serviceconfigurationid: ::windows_sys::core::HSTRING, sessiontemplatename: ::windows_sys::core::HSTRING, sessionid: ::windows_sys::core::HSTRING, invitationdisplaytext: ::windows_sys::core::HSTRING, customactivationcontext: ::windows_sys::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn ShowPlayerPickerUI(promptdisplaytext: ::windows_sys::core::HSTRING, xuids: *const ::windows_sys::core::HSTRING, xuidscount: usize, preselectedxuids: *const ::windows_sys::core::HSTRING, preselectedxuidscount: usize, minselectioncount: usize, maxselectioncount: usize, completionroutine: PlayerPickerUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn ShowPlayerPickerUIForUser(user: ::windows_sys::core::IInspectable, promptdisplaytext: ::windows_sys::core::HSTRING, xuids: *const ::windows_sys::core::HSTRING, xuidscount: usize, preselectedxuids: *const ::windows_sys::core::HSTRING, preselectedxuidscount: usize, minselectioncount: usize, maxselectioncount: usize, completionroutine: PlayerPickerUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn ShowProfileCardUI(targetuserxuid: ::windows_sys::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn ShowProfileCardUIForUser(user: ::windows_sys::core::IInspectable, targetuserxuid: ::windows_sys::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn ShowTitleAchievementsUI(titleid: u32, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn ShowTitleAchievementsUIForUser(user: ::windows_sys::core::IInspectable, titleid: u32, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn ShowUserSettingsUI(completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn ShowUserSettingsUIForUser(user: ::windows_sys::core::IInspectable, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TryCancelPendingGameUI() -> super::Foundation::BOOL;
}
pub const GAMESTATS_OPEN_CREATED: i32 = 0i32;
pub const GAMESTATS_OPEN_OPENED: i32 = 1i32;
pub const GAMESTATS_OPEN_OPENORCREATE: i32 = 0i32;
pub const GAMESTATS_OPEN_OPENONLY: i32 = 1i32;
pub const GIS_NOT_INSTALLED: i32 = 1i32;
pub const GIS_CURRENT_USER: i32 = 2i32;
pub const GIS_ALL_USERS: i32 = 3i32;
pub const GAMING_DEVICE_DEVICE_ID_NONE: i32 = 0i32;
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE: i32 = 1988865574i32;
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE_S: i32 = 712204761i32;
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE_X: i32 = 1523980231i32;
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE_X_DEVKIT: i32 = 284675555i32;
#[repr(C)]
pub struct GAMING_DEVICE_MODEL_INFORMATION {
    pub vendorId: GAMING_DEVICE_VENDOR_ID,
    pub deviceId: GAMING_DEVICE_DEVICE_ID,
}
impl ::core::marker::Copy for GAMING_DEVICE_MODEL_INFORMATION {}
impl ::core::clone::Clone for GAMING_DEVICE_MODEL_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const GAMING_DEVICE_VENDOR_ID_NONE: i32 = 0i32;
pub const GAMING_DEVICE_VENDOR_ID_MICROSOFT: i32 = -1024700366i32;
pub const GameExplorer: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2589895056, data2: 12340, data3: 19823, data4: [145, 40, 1, 243, 198, 16, 34, 188] };
pub const GameStatistics: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3687340588,
    data2: 49372,
    data3: 18785,
    data4: [182, 226, 210, 139, 98, 193, 26, 212],
};
pub type GameUICompletionRoutine = unsafe extern "system" fn(returncode: ::windows_sys::core::HRESULT, context: *const ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGameExplorer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGameExplorer {}
impl ::core::clone::Clone for IGameExplorer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGameExplorer2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGameExplorer2 {}
impl ::core::clone::Clone for IGameExplorer2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGameStatistics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGameStatistics {}
impl ::core::clone::Clone for IGameStatistics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGameStatisticsMgr(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGameStatisticsMgr {}
impl ::core::clone::Clone for IGameStatisticsMgr {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXblIdpAuthManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXblIdpAuthManager {}
impl ::core::clone::Clone for IXblIdpAuthManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXblIdpAuthTokenResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXblIdpAuthTokenResult {}
impl ::core::clone::Clone for IXblIdpAuthTokenResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXblIdpAuthTokenResult2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXblIdpAuthTokenResult2 {}
impl ::core::clone::Clone for IXblIdpAuthTokenResult2 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const XPRIVILEGE_BROADCAST: i32 = 190i32;
pub const XPRIVILEGE_VIEW_FRIENDS_LIST: i32 = 197i32;
pub const XPRIVILEGE_GAME_DVR: i32 = 198i32;
pub const XPRIVILEGE_SHARE_KINECT_CONTENT: i32 = 199i32;
pub const XPRIVILEGE_MULTIPLAYER_PARTIES: i32 = 203i32;
pub const XPRIVILEGE_COMMUNICATION_VOICE_INGAME: i32 = 205i32;
pub const XPRIVILEGE_COMMUNICATION_VOICE_SKYPE: i32 = 206i32;
pub const XPRIVILEGE_CLOUD_GAMING_MANAGE_SESSION: i32 = 207i32;
pub const XPRIVILEGE_CLOUD_GAMING_JOIN_SESSION: i32 = 208i32;
pub const XPRIVILEGE_CLOUD_SAVED_GAMES: i32 = 209i32;
pub const XPRIVILEGE_SHARE_CONTENT: i32 = 211i32;
pub const XPRIVILEGE_PREMIUM_CONTENT: i32 = 214i32;
pub const XPRIVILEGE_SUBSCRIPTION_CONTENT: i32 = 219i32;
pub const XPRIVILEGE_SOCIAL_NETWORK_SHARING: i32 = 220i32;
pub const XPRIVILEGE_PREMIUM_VIDEO: i32 = 224i32;
pub const XPRIVILEGE_VIDEO_COMMUNICATIONS: i32 = 235i32;
pub const XPRIVILEGE_PURCHASE_CONTENT: i32 = 245i32;
pub const XPRIVILEGE_USER_CREATED_CONTENT: i32 = 247i32;
pub const XPRIVILEGE_PROFILE_VIEWING: i32 = 249i32;
pub const XPRIVILEGE_COMMUNICATIONS: i32 = 252i32;
pub const XPRIVILEGE_MULTIPLAYER_SESSIONS: i32 = 254i32;
pub const XPRIVILEGE_ADD_FRIEND: i32 = 255i32;
pub type PlayerPickerUICompletionRoutine = unsafe extern "system" fn(returncode: ::windows_sys::core::HRESULT, context: *const ::core::ffi::c_void, selectedxuids: *const ::windows_sys::core::HSTRING, selectedxuidscount: usize);
pub const XBL_IDP_AUTH_TOKEN_STATUS_SUCCESS: i32 = 0i32;
pub const XBL_IDP_AUTH_TOKEN_STATUS_OFFLINE_SUCCESS: i32 = 1i32;
pub const XBL_IDP_AUTH_TOKEN_STATUS_NO_ACCOUNT_SET: i32 = 2i32;
pub const XBL_IDP_AUTH_TOKEN_STATUS_LOAD_MSA_ACCOUNT_FAILED: i32 = 3i32;
pub const XBL_IDP_AUTH_TOKEN_STATUS_XBOX_VETO: i32 = 4i32;
pub const XBL_IDP_AUTH_TOKEN_STATUS_MSA_INTERRUPT: i32 = 5i32;
pub const XBL_IDP_AUTH_TOKEN_STATUS_OFFLINE_NO_CONSENT: i32 = 6i32;
pub const XBL_IDP_AUTH_TOKEN_STATUS_VIEW_NOT_SET: i32 = 7i32;
pub const XBL_IDP_AUTH_TOKEN_STATUS_UNKNOWN: i32 = -1i32;
pub const XblIdpAuthManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3458421579,
    data2: 22232,
    data3: 18808,
    data4: [134, 162, 126, 229, 112, 100, 4, 104],
};
pub const XblIdpAuthTokenResult: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2672374849,
    data2: 29770,
    data3: 16652,
    data4: [174, 43, 154, 34, 247, 199, 115, 31],
};
