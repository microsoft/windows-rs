#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckGamingPrivilegeSilently(privilegeid: u32, scope: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, policy: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, hasprivilege: *mut super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckGamingPrivilegeSilentlyForUser(user: ::windows::runtime::RawPtr, privilegeid: u32, scope: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, policy: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, hasprivilege: *mut super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn CheckGamingPrivilegeWithUI(privilegeid: u32, scope: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, policy: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, friendlymessage: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, completionroutine: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn CheckGamingPrivilegeWithUIForUser(user: ::windows::runtime::RawPtr, privilegeid: u32, scope: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, policy: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, friendlymessage: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, completionroutine: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn GetExpandedResourceExclusiveCpuCount(exclusivecpucount: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn GetGamingDeviceModelInformation(information: *mut GAMING_DEVICE_MODEL_INFORMATION) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HasExpandedResources(hasexpandedresources: *mut super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ProcessPendingGameUI(waitforcompletion: super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn ReleaseExclusiveCpuSets() -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn ShowChangeFriendRelationshipUI(targetuserxuid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, completionroutine: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn ShowChangeFriendRelationshipUIForUser(user: ::windows::runtime::RawPtr, targetuserxuid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, completionroutine: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn ShowCustomizeUserProfileUI(completionroutine: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn ShowCustomizeUserProfileUIForUser(user: ::windows::runtime::RawPtr, completionroutine: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn ShowFindFriendsUI(completionroutine: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn ShowFindFriendsUIForUser(user: ::windows::runtime::RawPtr, completionroutine: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn ShowGameInfoUI(titleid: u32, completionroutine: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn ShowGameInfoUIForUser(user: ::windows::runtime::RawPtr, titleid: u32, completionroutine: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn ShowGameInviteUI(serviceconfigurationid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, sessiontemplatename: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, sessionid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, invitationdisplaytext: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, completionroutine: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn ShowGameInviteUIForUser(user: ::windows::runtime::RawPtr, serviceconfigurationid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, sessiontemplatename: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, sessionid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, invitationdisplaytext: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, completionroutine: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn ShowGameInviteUIWithContext(
        serviceconfigurationid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        sessiontemplatename: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        sessionid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        invitationdisplaytext: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        customactivationcontext: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        completionroutine: ::windows::runtime::RawPtr,
        context: *const ::core::ffi::c_void,
    ) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn ShowGameInviteUIWithContextForUser(
        user: ::windows::runtime::RawPtr,
        serviceconfigurationid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        sessiontemplatename: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        sessionid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        invitationdisplaytext: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        customactivationcontext: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        completionroutine: ::windows::runtime::RawPtr,
        context: *const ::core::ffi::c_void,
    ) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn ShowPlayerPickerUI(promptdisplaytext: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, xuids: *const ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, xuidscount: usize, preselectedxuids: *const ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, preselectedxuidscount: usize, minselectioncount: usize, maxselectioncount: usize, completionroutine: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn ShowPlayerPickerUIForUser(
        user: ::windows::runtime::RawPtr,
        promptdisplaytext: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        xuids: *const ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        xuidscount: usize,
        preselectedxuids: *const ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        preselectedxuidscount: usize,
        minselectioncount: usize,
        maxselectioncount: usize,
        completionroutine: ::windows::runtime::RawPtr,
        context: *const ::core::ffi::c_void,
    ) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn ShowProfileCardUI(targetuserxuid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, completionroutine: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn ShowProfileCardUIForUser(user: ::windows::runtime::RawPtr, targetuserxuid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, completionroutine: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn ShowTitleAchievementsUI(titleid: u32, completionroutine: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn ShowTitleAchievementsUIForUser(user: ::windows::runtime::RawPtr, titleid: u32, completionroutine: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn ShowUserSettingsUI(completionroutine: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn ShowUserSettingsUIForUser(user: ::windows::runtime::RawPtr, completionroutine: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TryCancelPendingGameUI() -> super::Foundation::BOOL;
}
