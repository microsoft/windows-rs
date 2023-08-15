#[doc = "*Required features: `\"Win32_Gaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CheckGamingPrivilegeSilently(privilegeid: u32, scope: &::windows_core::HSTRING, policy: &::windows_core::HSTRING) -> ::windows_core::Result<super::Foundation::BOOL> {
    ::windows_targets::link!("api-ms-win-gaming-tcui-l1-1-1.dll" "system" fn CheckGamingPrivilegeSilently(privilegeid : u32, scope : ::std::mem::MaybeUninit <::windows_core::HSTRING >, policy : ::std::mem::MaybeUninit <::windows_core::HSTRING >, hasprivilege : *mut super::Foundation:: BOOL) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    CheckGamingPrivilegeSilently(privilegeid, ::core::mem::transmute_copy(scope), ::core::mem::transmute_copy(policy), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Gaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CheckGamingPrivilegeSilentlyForUser<P0>(user: P0, privilegeid: u32, scope: &::windows_core::HSTRING, policy: &::windows_core::HSTRING) -> ::windows_core::Result<super::Foundation::BOOL>
where
    P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
{
    ::windows_targets::link!("api-ms-win-gaming-tcui-l1-1-2.dll" "system" fn CheckGamingPrivilegeSilentlyForUser(user : * mut::core::ffi::c_void, privilegeid : u32, scope : ::std::mem::MaybeUninit <::windows_core::HSTRING >, policy : ::std::mem::MaybeUninit <::windows_core::HSTRING >, hasprivilege : *mut super::Foundation:: BOOL) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    CheckGamingPrivilegeSilentlyForUser(user.into_param().abi(), privilegeid, ::core::mem::transmute_copy(scope), ::core::mem::transmute_copy(policy), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn CheckGamingPrivilegeWithUI(privilegeid: u32, scope: &::windows_core::HSTRING, policy: &::windows_core::HSTRING, friendlymessage: &::windows_core::HSTRING, completionroutine: GameUICompletionRoutine, context: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows_core::Result<()> {
    ::windows_targets::link!("api-ms-win-gaming-tcui-l1-1-1.dll" "system" fn CheckGamingPrivilegeWithUI(privilegeid : u32, scope : ::std::mem::MaybeUninit <::windows_core::HSTRING >, policy : ::std::mem::MaybeUninit <::windows_core::HSTRING >, friendlymessage : ::std::mem::MaybeUninit <::windows_core::HSTRING >, completionroutine : GameUICompletionRoutine, context : *const ::core::ffi::c_void) -> ::windows_core::HRESULT);
    CheckGamingPrivilegeWithUI(privilegeid, ::core::mem::transmute_copy(scope), ::core::mem::transmute_copy(policy), ::core::mem::transmute_copy(friendlymessage), completionroutine, ::core::mem::transmute(context.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn CheckGamingPrivilegeWithUIForUser<P0>(user: P0, privilegeid: u32, scope: &::windows_core::HSTRING, policy: &::windows_core::HSTRING, friendlymessage: &::windows_core::HSTRING, completionroutine: GameUICompletionRoutine, context: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
{
    ::windows_targets::link!("api-ms-win-gaming-tcui-l1-1-2.dll" "system" fn CheckGamingPrivilegeWithUIForUser(user : * mut::core::ffi::c_void, privilegeid : u32, scope : ::std::mem::MaybeUninit <::windows_core::HSTRING >, policy : ::std::mem::MaybeUninit <::windows_core::HSTRING >, friendlymessage : ::std::mem::MaybeUninit <::windows_core::HSTRING >, completionroutine : GameUICompletionRoutine, context : *const ::core::ffi::c_void) -> ::windows_core::HRESULT);
    CheckGamingPrivilegeWithUIForUser(user.into_param().abi(), privilegeid, ::core::mem::transmute_copy(scope), ::core::mem::transmute_copy(policy), ::core::mem::transmute_copy(friendlymessage), completionroutine, ::core::mem::transmute(context.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn GetExpandedResourceExclusiveCpuCount() -> ::windows_core::Result<u32> {
    ::windows_targets::link!("api-ms-win-gaming-expandedresources-l1-1-0.dll" "system" fn GetExpandedResourceExclusiveCpuCount(exclusivecpucount : *mut u32) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    GetExpandedResourceExclusiveCpuCount(&mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn GetGamingDeviceModelInformation() -> ::windows_core::Result<GAMING_DEVICE_MODEL_INFORMATION> {
    ::windows_targets::link!("api-ms-win-gaming-deviceinformation-l1-1-0.dll" "system" fn GetGamingDeviceModelInformation(information : *mut GAMING_DEVICE_MODEL_INFORMATION) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    GetGamingDeviceModelInformation(&mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Gaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HasExpandedResources() -> ::windows_core::Result<super::Foundation::BOOL> {
    ::windows_targets::link!("api-ms-win-gaming-expandedresources-l1-1-0.dll" "system" fn HasExpandedResources(hasexpandedresources : *mut super::Foundation:: BOOL) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    HasExpandedResources(&mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Gaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ProcessPendingGameUI<P0>(waitforcompletion: P0) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::Foundation::BOOL>,
{
    ::windows_targets::link!("api-ms-win-gaming-tcui-l1-1-0.dll" "system" fn ProcessPendingGameUI(waitforcompletion : super::Foundation:: BOOL) -> ::windows_core::HRESULT);
    ProcessPendingGameUI(waitforcompletion.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ReleaseExclusiveCpuSets() -> ::windows_core::Result<()> {
    ::windows_targets::link!("api-ms-win-gaming-expandedresources-l1-1-0.dll" "system" fn ReleaseExclusiveCpuSets() -> ::windows_core::HRESULT);
    ReleaseExclusiveCpuSets().ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowChangeFriendRelationshipUI(targetuserxuid: &::windows_core::HSTRING, completionroutine: GameUICompletionRoutine, context: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows_core::Result<()> {
    ::windows_targets::link!("api-ms-win-gaming-tcui-l1-1-0.dll" "system" fn ShowChangeFriendRelationshipUI(targetuserxuid : ::std::mem::MaybeUninit <::windows_core::HSTRING >, completionroutine : GameUICompletionRoutine, context : *const ::core::ffi::c_void) -> ::windows_core::HRESULT);
    ShowChangeFriendRelationshipUI(::core::mem::transmute_copy(targetuserxuid), completionroutine, ::core::mem::transmute(context.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowChangeFriendRelationshipUIForUser<P0>(user: P0, targetuserxuid: &::windows_core::HSTRING, completionroutine: GameUICompletionRoutine, context: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
{
    ::windows_targets::link!("api-ms-win-gaming-tcui-l1-1-2.dll" "system" fn ShowChangeFriendRelationshipUIForUser(user : * mut::core::ffi::c_void, targetuserxuid : ::std::mem::MaybeUninit <::windows_core::HSTRING >, completionroutine : GameUICompletionRoutine, context : *const ::core::ffi::c_void) -> ::windows_core::HRESULT);
    ShowChangeFriendRelationshipUIForUser(user.into_param().abi(), ::core::mem::transmute_copy(targetuserxuid), completionroutine, ::core::mem::transmute(context.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowCustomizeUserProfileUI(completionroutine: GameUICompletionRoutine, context: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows_core::Result<()> {
    ::windows_targets::link!("api-ms-win-gaming-tcui-l1-1-4.dll" "system" fn ShowCustomizeUserProfileUI(completionroutine : GameUICompletionRoutine, context : *const ::core::ffi::c_void) -> ::windows_core::HRESULT);
    ShowCustomizeUserProfileUI(completionroutine, ::core::mem::transmute(context.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowCustomizeUserProfileUIForUser<P0>(user: P0, completionroutine: GameUICompletionRoutine, context: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
{
    ::windows_targets::link!("api-ms-win-gaming-tcui-l1-1-4.dll" "system" fn ShowCustomizeUserProfileUIForUser(user : * mut::core::ffi::c_void, completionroutine : GameUICompletionRoutine, context : *const ::core::ffi::c_void) -> ::windows_core::HRESULT);
    ShowCustomizeUserProfileUIForUser(user.into_param().abi(), completionroutine, ::core::mem::transmute(context.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowFindFriendsUI(completionroutine: GameUICompletionRoutine, context: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows_core::Result<()> {
    ::windows_targets::link!("api-ms-win-gaming-tcui-l1-1-4.dll" "system" fn ShowFindFriendsUI(completionroutine : GameUICompletionRoutine, context : *const ::core::ffi::c_void) -> ::windows_core::HRESULT);
    ShowFindFriendsUI(completionroutine, ::core::mem::transmute(context.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowFindFriendsUIForUser<P0>(user: P0, completionroutine: GameUICompletionRoutine, context: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
{
    ::windows_targets::link!("api-ms-win-gaming-tcui-l1-1-4.dll" "system" fn ShowFindFriendsUIForUser(user : * mut::core::ffi::c_void, completionroutine : GameUICompletionRoutine, context : *const ::core::ffi::c_void) -> ::windows_core::HRESULT);
    ShowFindFriendsUIForUser(user.into_param().abi(), completionroutine, ::core::mem::transmute(context.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowGameInfoUI(titleid: u32, completionroutine: GameUICompletionRoutine, context: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows_core::Result<()> {
    ::windows_targets::link!("api-ms-win-gaming-tcui-l1-1-4.dll" "system" fn ShowGameInfoUI(titleid : u32, completionroutine : GameUICompletionRoutine, context : *const ::core::ffi::c_void) -> ::windows_core::HRESULT);
    ShowGameInfoUI(titleid, completionroutine, ::core::mem::transmute(context.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowGameInfoUIForUser<P0>(user: P0, titleid: u32, completionroutine: GameUICompletionRoutine, context: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
{
    ::windows_targets::link!("api-ms-win-gaming-tcui-l1-1-4.dll" "system" fn ShowGameInfoUIForUser(user : * mut::core::ffi::c_void, titleid : u32, completionroutine : GameUICompletionRoutine, context : *const ::core::ffi::c_void) -> ::windows_core::HRESULT);
    ShowGameInfoUIForUser(user.into_param().abi(), titleid, completionroutine, ::core::mem::transmute(context.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowGameInviteUI(serviceconfigurationid: &::windows_core::HSTRING, sessiontemplatename: &::windows_core::HSTRING, sessionid: &::windows_core::HSTRING, invitationdisplaytext: &::windows_core::HSTRING, completionroutine: GameUICompletionRoutine, context: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows_core::Result<()> {
    ::windows_targets::link!("api-ms-win-gaming-tcui-l1-1-0.dll" "system" fn ShowGameInviteUI(serviceconfigurationid : ::std::mem::MaybeUninit <::windows_core::HSTRING >, sessiontemplatename : ::std::mem::MaybeUninit <::windows_core::HSTRING >, sessionid : ::std::mem::MaybeUninit <::windows_core::HSTRING >, invitationdisplaytext : ::std::mem::MaybeUninit <::windows_core::HSTRING >, completionroutine : GameUICompletionRoutine, context : *const ::core::ffi::c_void) -> ::windows_core::HRESULT);
    ShowGameInviteUI(::core::mem::transmute_copy(serviceconfigurationid), ::core::mem::transmute_copy(sessiontemplatename), ::core::mem::transmute_copy(sessionid), ::core::mem::transmute_copy(invitationdisplaytext), completionroutine, ::core::mem::transmute(context.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowGameInviteUIForUser<P0>(user: P0, serviceconfigurationid: &::windows_core::HSTRING, sessiontemplatename: &::windows_core::HSTRING, sessionid: &::windows_core::HSTRING, invitationdisplaytext: &::windows_core::HSTRING, completionroutine: GameUICompletionRoutine, context: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
{
    ::windows_targets::link!("api-ms-win-gaming-tcui-l1-1-2.dll" "system" fn ShowGameInviteUIForUser(user : * mut::core::ffi::c_void, serviceconfigurationid : ::std::mem::MaybeUninit <::windows_core::HSTRING >, sessiontemplatename : ::std::mem::MaybeUninit <::windows_core::HSTRING >, sessionid : ::std::mem::MaybeUninit <::windows_core::HSTRING >, invitationdisplaytext : ::std::mem::MaybeUninit <::windows_core::HSTRING >, completionroutine : GameUICompletionRoutine, context : *const ::core::ffi::c_void) -> ::windows_core::HRESULT);
    ShowGameInviteUIForUser(user.into_param().abi(), ::core::mem::transmute_copy(serviceconfigurationid), ::core::mem::transmute_copy(sessiontemplatename), ::core::mem::transmute_copy(sessionid), ::core::mem::transmute_copy(invitationdisplaytext), completionroutine, ::core::mem::transmute(context.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowGameInviteUIWithContext(serviceconfigurationid: &::windows_core::HSTRING, sessiontemplatename: &::windows_core::HSTRING, sessionid: &::windows_core::HSTRING, invitationdisplaytext: &::windows_core::HSTRING, customactivationcontext: &::windows_core::HSTRING, completionroutine: GameUICompletionRoutine, context: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows_core::Result<()> {
    ::windows_targets::link!("api-ms-win-gaming-tcui-l1-1-3.dll" "system" fn ShowGameInviteUIWithContext(serviceconfigurationid : ::std::mem::MaybeUninit <::windows_core::HSTRING >, sessiontemplatename : ::std::mem::MaybeUninit <::windows_core::HSTRING >, sessionid : ::std::mem::MaybeUninit <::windows_core::HSTRING >, invitationdisplaytext : ::std::mem::MaybeUninit <::windows_core::HSTRING >, customactivationcontext : ::std::mem::MaybeUninit <::windows_core::HSTRING >, completionroutine : GameUICompletionRoutine, context : *const ::core::ffi::c_void) -> ::windows_core::HRESULT);
    ShowGameInviteUIWithContext(::core::mem::transmute_copy(serviceconfigurationid), ::core::mem::transmute_copy(sessiontemplatename), ::core::mem::transmute_copy(sessionid), ::core::mem::transmute_copy(invitationdisplaytext), ::core::mem::transmute_copy(customactivationcontext), completionroutine, ::core::mem::transmute(context.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowGameInviteUIWithContextForUser<P0>(user: P0, serviceconfigurationid: &::windows_core::HSTRING, sessiontemplatename: &::windows_core::HSTRING, sessionid: &::windows_core::HSTRING, invitationdisplaytext: &::windows_core::HSTRING, customactivationcontext: &::windows_core::HSTRING, completionroutine: GameUICompletionRoutine, context: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
{
    ::windows_targets::link!("api-ms-win-gaming-tcui-l1-1-3.dll" "system" fn ShowGameInviteUIWithContextForUser(user : * mut::core::ffi::c_void, serviceconfigurationid : ::std::mem::MaybeUninit <::windows_core::HSTRING >, sessiontemplatename : ::std::mem::MaybeUninit <::windows_core::HSTRING >, sessionid : ::std::mem::MaybeUninit <::windows_core::HSTRING >, invitationdisplaytext : ::std::mem::MaybeUninit <::windows_core::HSTRING >, customactivationcontext : ::std::mem::MaybeUninit <::windows_core::HSTRING >, completionroutine : GameUICompletionRoutine, context : *const ::core::ffi::c_void) -> ::windows_core::HRESULT);
    ShowGameInviteUIWithContextForUser(user.into_param().abi(), ::core::mem::transmute_copy(serviceconfigurationid), ::core::mem::transmute_copy(sessiontemplatename), ::core::mem::transmute_copy(sessionid), ::core::mem::transmute_copy(invitationdisplaytext), ::core::mem::transmute_copy(customactivationcontext), completionroutine, ::core::mem::transmute(context.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowPlayerPickerUI(promptdisplaytext: &::windows_core::HSTRING, xuids: &[::windows_core::HSTRING], preselectedxuids: ::core::option::Option<&[::windows_core::HSTRING]>, minselectioncount: usize, maxselectioncount: usize, completionroutine: PlayerPickerUICompletionRoutine, context: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows_core::Result<()> {
    ::windows_targets::link!("api-ms-win-gaming-tcui-l1-1-0.dll" "system" fn ShowPlayerPickerUI(promptdisplaytext : ::std::mem::MaybeUninit <::windows_core::HSTRING >, xuids : *const ::std::mem::MaybeUninit <::windows_core::HSTRING >, xuidscount : usize, preselectedxuids : *const ::std::mem::MaybeUninit <::windows_core::HSTRING >, preselectedxuidscount : usize, minselectioncount : usize, maxselectioncount : usize, completionroutine : PlayerPickerUICompletionRoutine, context : *const ::core::ffi::c_void) -> ::windows_core::HRESULT);
    ShowPlayerPickerUI(::core::mem::transmute_copy(promptdisplaytext), ::core::mem::transmute(xuids.as_ptr()), xuids.len() as _, ::core::mem::transmute(preselectedxuids.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), preselectedxuids.as_deref().map_or(0, |slice| slice.len() as _), minselectioncount, maxselectioncount, completionroutine, ::core::mem::transmute(context.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowPlayerPickerUIForUser<P0>(user: P0, promptdisplaytext: &::windows_core::HSTRING, xuids: &[::windows_core::HSTRING], preselectedxuids: ::core::option::Option<&[::windows_core::HSTRING]>, minselectioncount: usize, maxselectioncount: usize, completionroutine: PlayerPickerUICompletionRoutine, context: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
{
    ::windows_targets::link!("api-ms-win-gaming-tcui-l1-1-2.dll" "system" fn ShowPlayerPickerUIForUser(user : * mut::core::ffi::c_void, promptdisplaytext : ::std::mem::MaybeUninit <::windows_core::HSTRING >, xuids : *const ::std::mem::MaybeUninit <::windows_core::HSTRING >, xuidscount : usize, preselectedxuids : *const ::std::mem::MaybeUninit <::windows_core::HSTRING >, preselectedxuidscount : usize, minselectioncount : usize, maxselectioncount : usize, completionroutine : PlayerPickerUICompletionRoutine, context : *const ::core::ffi::c_void) -> ::windows_core::HRESULT);
    ShowPlayerPickerUIForUser(user.into_param().abi(), ::core::mem::transmute_copy(promptdisplaytext), ::core::mem::transmute(xuids.as_ptr()), xuids.len() as _, ::core::mem::transmute(preselectedxuids.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), preselectedxuids.as_deref().map_or(0, |slice| slice.len() as _), minselectioncount, maxselectioncount, completionroutine, ::core::mem::transmute(context.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowProfileCardUI(targetuserxuid: &::windows_core::HSTRING, completionroutine: GameUICompletionRoutine, context: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows_core::Result<()> {
    ::windows_targets::link!("api-ms-win-gaming-tcui-l1-1-0.dll" "system" fn ShowProfileCardUI(targetuserxuid : ::std::mem::MaybeUninit <::windows_core::HSTRING >, completionroutine : GameUICompletionRoutine, context : *const ::core::ffi::c_void) -> ::windows_core::HRESULT);
    ShowProfileCardUI(::core::mem::transmute_copy(targetuserxuid), completionroutine, ::core::mem::transmute(context.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowProfileCardUIForUser<P0>(user: P0, targetuserxuid: &::windows_core::HSTRING, completionroutine: GameUICompletionRoutine, context: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
{
    ::windows_targets::link!("api-ms-win-gaming-tcui-l1-1-2.dll" "system" fn ShowProfileCardUIForUser(user : * mut::core::ffi::c_void, targetuserxuid : ::std::mem::MaybeUninit <::windows_core::HSTRING >, completionroutine : GameUICompletionRoutine, context : *const ::core::ffi::c_void) -> ::windows_core::HRESULT);
    ShowProfileCardUIForUser(user.into_param().abi(), ::core::mem::transmute_copy(targetuserxuid), completionroutine, ::core::mem::transmute(context.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowTitleAchievementsUI(titleid: u32, completionroutine: GameUICompletionRoutine, context: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows_core::Result<()> {
    ::windows_targets::link!("api-ms-win-gaming-tcui-l1-1-0.dll" "system" fn ShowTitleAchievementsUI(titleid : u32, completionroutine : GameUICompletionRoutine, context : *const ::core::ffi::c_void) -> ::windows_core::HRESULT);
    ShowTitleAchievementsUI(titleid, completionroutine, ::core::mem::transmute(context.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowTitleAchievementsUIForUser<P0>(user: P0, titleid: u32, completionroutine: GameUICompletionRoutine, context: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
{
    ::windows_targets::link!("api-ms-win-gaming-tcui-l1-1-2.dll" "system" fn ShowTitleAchievementsUIForUser(user : * mut::core::ffi::c_void, titleid : u32, completionroutine : GameUICompletionRoutine, context : *const ::core::ffi::c_void) -> ::windows_core::HRESULT);
    ShowTitleAchievementsUIForUser(user.into_param().abi(), titleid, completionroutine, ::core::mem::transmute(context.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowUserSettingsUI(completionroutine: GameUICompletionRoutine, context: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows_core::Result<()> {
    ::windows_targets::link!("api-ms-win-gaming-tcui-l1-1-4.dll" "system" fn ShowUserSettingsUI(completionroutine : GameUICompletionRoutine, context : *const ::core::ffi::c_void) -> ::windows_core::HRESULT);
    ShowUserSettingsUI(completionroutine, ::core::mem::transmute(context.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowUserSettingsUIForUser<P0>(user: P0, completionroutine: GameUICompletionRoutine, context: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
{
    ::windows_targets::link!("api-ms-win-gaming-tcui-l1-1-4.dll" "system" fn ShowUserSettingsUIForUser(user : * mut::core::ffi::c_void, completionroutine : GameUICompletionRoutine, context : *const ::core::ffi::c_void) -> ::windows_core::HRESULT);
    ShowUserSettingsUIForUser(user.into_param().abi(), completionroutine, ::core::mem::transmute(context.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TryCancelPendingGameUI() -> super::Foundation::BOOL {
    ::windows_targets::link!("api-ms-win-gaming-tcui-l1-1-0.dll" "system" fn TryCancelPendingGameUI() -> super::Foundation:: BOOL);
    TryCancelPendingGameUI()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[repr(transparent)]
pub struct IGameExplorer(::windows_core::IUnknown);
impl IGameExplorer {
    pub unsafe fn AddGame<P0, P1>(&self, bstrgdfbinarypath: P0, bstrgameinstalldirectory: P1, installscope: GAME_INSTALL_SCOPE, pguidinstanceid: *mut ::windows_core::GUID) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).AddGame)(::windows_core::Interface::as_raw(self), bstrgdfbinarypath.into_param().abi(), bstrgameinstalldirectory.into_param().abi(), installscope, pguidinstanceid).ok()
    }
    pub unsafe fn RemoveGame(&self, guidinstanceid: ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveGame)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(guidinstanceid)).ok()
    }
    pub unsafe fn UpdateGame(&self, guidinstanceid: ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UpdateGame)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(guidinstanceid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VerifyAccess<P0>(&self, bstrgdfbinarypath: P0) -> ::windows_core::Result<super::Foundation::BOOL>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).VerifyAccess)(::windows_core::Interface::as_raw(self), bstrgdfbinarypath.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IGameExplorer, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IGameExplorer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGameExplorer {}
impl ::core::fmt::Debug for IGameExplorer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGameExplorer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IGameExplorer {
    type Vtable = IGameExplorer_Vtbl;
}
impl ::core::clone::Clone for IGameExplorer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGameExplorer {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe7b2fb72_d728_49b3_a5f2_18ebf5f1349e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameExplorer_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub AddGame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrgdfbinarypath: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrgameinstalldirectory: ::std::mem::MaybeUninit<::windows_core::BSTR>, installscope: GAME_INSTALL_SCOPE, pguidinstanceid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub RemoveGame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidinstanceid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub UpdateGame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidinstanceid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub VerifyAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrgdfbinarypath: ::std::mem::MaybeUninit<::windows_core::BSTR>, pfhasaccess: *mut super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    VerifyAccess: usize,
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[repr(transparent)]
pub struct IGameExplorer2(::windows_core::IUnknown);
impl IGameExplorer2 {
    pub unsafe fn InstallGame<P0, P1>(&self, binarygdfpath: P0, installdirectory: P1, installscope: GAME_INSTALL_SCOPE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).InstallGame)(::windows_core::Interface::as_raw(self), binarygdfpath.into_param().abi(), installdirectory.into_param().abi(), installscope).ok()
    }
    pub unsafe fn UninstallGame<P0>(&self, binarygdfpath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).UninstallGame)(::windows_core::Interface::as_raw(self), binarygdfpath.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CheckAccess<P0>(&self, binarygdfpath: P0) -> ::windows_core::Result<super::Foundation::BOOL>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CheckAccess)(::windows_core::Interface::as_raw(self), binarygdfpath.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IGameExplorer2, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IGameExplorer2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGameExplorer2 {}
impl ::core::fmt::Debug for IGameExplorer2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGameExplorer2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IGameExplorer2 {
    type Vtable = IGameExplorer2_Vtbl;
}
impl ::core::clone::Clone for IGameExplorer2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGameExplorer2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x86874aa7_a1ed_450d_a7eb_b89e20b2fff3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameExplorer2_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub InstallGame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, binarygdfpath: ::windows_core::PCWSTR, installdirectory: ::windows_core::PCWSTR, installscope: GAME_INSTALL_SCOPE) -> ::windows_core::HRESULT,
    pub UninstallGame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, binarygdfpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CheckAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, binarygdfpath: ::windows_core::PCWSTR, phasaccess: *mut super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CheckAccess: usize,
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[repr(transparent)]
pub struct IGameStatistics(::windows_core::IUnknown);
impl IGameStatistics {
    pub unsafe fn GetMaxCategoryLength(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetMaxCategoryLength)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMaxNameLength(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetMaxNameLength)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMaxValueLength(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetMaxValueLength)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMaxCategories(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetMaxCategories)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMaxStatsPerCategory(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetMaxStatsPerCategory)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCategoryTitle<P0>(&self, categoryindex: u16, title: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetCategoryTitle)(::windows_core::Interface::as_raw(self), categoryindex, title.into_param().abi()).ok()
    }
    pub unsafe fn GetCategoryTitle(&self, categoryindex: u16) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCategoryTitle)(::windows_core::Interface::as_raw(self), categoryindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStatistic(&self, categoryindex: u16, statindex: u16, pname: ::core::option::Option<*mut ::windows_core::PWSTR>, pvalue: ::core::option::Option<*mut ::windows_core::PWSTR>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetStatistic)(::windows_core::Interface::as_raw(self), categoryindex, statindex, ::core::mem::transmute(pname.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pvalue.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetStatistic<P0, P1>(&self, categoryindex: u16, statindex: u16, name: P0, value: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetStatistic)(::windows_core::Interface::as_raw(self), categoryindex, statindex, name.into_param().abi(), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Save<P0>(&self, trackchanges: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).Save)(::windows_core::Interface::as_raw(self), trackchanges.into_param().abi()).ok()
    }
    pub unsafe fn SetLastPlayedCategory(&self, categoryindex: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLastPlayedCategory)(::windows_core::Interface::as_raw(self), categoryindex).ok()
    }
    pub unsafe fn GetLastPlayedCategory(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLastPlayedCategory)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IGameStatistics, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IGameStatistics {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGameStatistics {}
impl ::core::fmt::Debug for IGameStatistics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGameStatistics").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IGameStatistics {
    type Vtable = IGameStatistics_Vtbl;
}
impl ::core::clone::Clone for IGameStatistics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGameStatistics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3887c9ca_04a0_42ae_bc4c_5fa6c7721145);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameStatistics_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetMaxCategoryLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cch: *mut u32) -> ::windows_core::HRESULT,
    pub GetMaxNameLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cch: *mut u32) -> ::windows_core::HRESULT,
    pub GetMaxValueLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cch: *mut u32) -> ::windows_core::HRESULT,
    pub GetMaxCategories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmax: *mut u16) -> ::windows_core::HRESULT,
    pub GetMaxStatsPerCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmax: *mut u16) -> ::windows_core::HRESULT,
    pub SetCategoryTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, categoryindex: u16, title: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetCategoryTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, categoryindex: u16, ptitle: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetStatistic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, categoryindex: u16, statindex: u16, pname: *mut ::windows_core::PWSTR, pvalue: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetStatistic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, categoryindex: u16, statindex: u16, name: ::windows_core::PCWSTR, value: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, trackchanges: super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Save: usize,
    pub SetLastPlayedCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, categoryindex: u32) -> ::windows_core::HRESULT,
    pub GetLastPlayedCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcategoryindex: *mut u32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[repr(transparent)]
pub struct IGameStatisticsMgr(::windows_core::IUnknown);
impl IGameStatisticsMgr {
    pub unsafe fn GetGameStatistics<P0>(&self, gdfbinarypath: P0, opentype: GAMESTATS_OPEN_TYPE, popenresult: *mut GAMESTATS_OPEN_RESULT, ppistats: *mut ::core::option::Option<IGameStatistics>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).GetGameStatistics)(::windows_core::Interface::as_raw(self), gdfbinarypath.into_param().abi(), opentype, popenresult, ::core::mem::transmute(ppistats)).ok()
    }
    pub unsafe fn RemoveGameStatistics<P0>(&self, gdfbinarypath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).RemoveGameStatistics)(::windows_core::Interface::as_raw(self), gdfbinarypath.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IGameStatisticsMgr, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IGameStatisticsMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGameStatisticsMgr {}
impl ::core::fmt::Debug for IGameStatisticsMgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGameStatisticsMgr").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IGameStatisticsMgr {
    type Vtable = IGameStatisticsMgr_Vtbl;
}
impl ::core::clone::Clone for IGameStatisticsMgr {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGameStatisticsMgr {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaff3ea11_e70e_407d_95dd_35e612c41ce2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameStatisticsMgr_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetGameStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gdfbinarypath: ::windows_core::PCWSTR, opentype: GAMESTATS_OPEN_TYPE, popenresult: *mut GAMESTATS_OPEN_RESULT, ppistats: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveGameStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gdfbinarypath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[repr(transparent)]
pub struct IXblIdpAuthManager(::windows_core::IUnknown);
impl IXblIdpAuthManager {
    pub unsafe fn SetGamerAccount<P0, P1>(&self, msaaccountid: P0, xuid: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetGamerAccount)(::windows_core::Interface::as_raw(self), msaaccountid.into_param().abi(), xuid.into_param().abi()).ok()
    }
    pub unsafe fn GetGamerAccount(&self, msaaccountid: *mut ::windows_core::PWSTR, xuid: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetGamerAccount)(::windows_core::Interface::as_raw(self), msaaccountid, xuid).ok()
    }
    pub unsafe fn SetAppViewInitialized<P0, P1>(&self, appsid: P0, msaaccountid: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetAppViewInitialized)(::windows_core::Interface::as_raw(self), appsid.into_param().abi(), msaaccountid.into_param().abi()).ok()
    }
    pub unsafe fn GetEnvironment(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetEnvironment)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSandbox(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSandbox)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTokenAndSignatureWithTokenResult<P0, P1, P2, P3, P4, P5, P6, P7>(&self, msaaccountid: P0, appsid: P1, msatarget: P2, msapolicy: P3, httpmethod: P4, uri: P5, headers: P6, body: &[u8], forcerefresh: P7) -> ::windows_core::Result<IXblIdpAuthTokenResult>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P4: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P5: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P6: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P7: ::windows_core::IntoParam<super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTokenAndSignatureWithTokenResult)(::windows_core::Interface::as_raw(self), msaaccountid.into_param().abi(), appsid.into_param().abi(), msatarget.into_param().abi(), msapolicy.into_param().abi(), httpmethod.into_param().abi(), uri.into_param().abi(), headers.into_param().abi(), ::core::mem::transmute(body.as_ptr()), body.len() as _, forcerefresh.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXblIdpAuthManager, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IXblIdpAuthManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXblIdpAuthManager {}
impl ::core::fmt::Debug for IXblIdpAuthManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXblIdpAuthManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IXblIdpAuthManager {
    type Vtable = IXblIdpAuthManager_Vtbl;
}
impl ::core::clone::Clone for IXblIdpAuthManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IXblIdpAuthManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeb5ddb08_8bbf_449b_ac21_b02ddeb3b136);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXblIdpAuthManager_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SetGamerAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, msaaccountid: ::windows_core::PCWSTR, xuid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetGamerAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, msaaccountid: *mut ::windows_core::PWSTR, xuid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetAppViewInitialized: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appsid: ::windows_core::PCWSTR, msaaccountid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetEnvironment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, environment: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetSandbox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sandbox: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTokenAndSignatureWithTokenResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, msaaccountid: ::windows_core::PCWSTR, appsid: ::windows_core::PCWSTR, msatarget: ::windows_core::PCWSTR, msapolicy: ::windows_core::PCWSTR, httpmethod: ::windows_core::PCWSTR, uri: ::windows_core::PCWSTR, headers: ::windows_core::PCWSTR, body: *const u8, bodysize: u32, forcerefresh: super::Foundation::BOOL, result: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTokenAndSignatureWithTokenResult: usize,
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[repr(transparent)]
pub struct IXblIdpAuthManager2(::windows_core::IUnknown);
impl IXblIdpAuthManager2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUserlessTokenAndSignatureWithTokenResult<P0, P1, P2, P3, P4, P5, P6>(&self, appsid: P0, msatarget: P1, msapolicy: P2, httpmethod: P3, uri: P4, headers: P5, body: &[u8], forcerefresh: P6) -> ::windows_core::Result<IXblIdpAuthTokenResult>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P4: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P5: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P6: ::windows_core::IntoParam<super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetUserlessTokenAndSignatureWithTokenResult)(::windows_core::Interface::as_raw(self), appsid.into_param().abi(), msatarget.into_param().abi(), msapolicy.into_param().abi(), httpmethod.into_param().abi(), uri.into_param().abi(), headers.into_param().abi(), ::core::mem::transmute(body.as_ptr()), body.len() as _, forcerefresh.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXblIdpAuthManager2, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IXblIdpAuthManager2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXblIdpAuthManager2 {}
impl ::core::fmt::Debug for IXblIdpAuthManager2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXblIdpAuthManager2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IXblIdpAuthManager2 {
    type Vtable = IXblIdpAuthManager2_Vtbl;
}
impl ::core::clone::Clone for IXblIdpAuthManager2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IXblIdpAuthManager2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbf8c0950_8389_43dd_9a76_a19728ec5dc5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXblIdpAuthManager2_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetUserlessTokenAndSignatureWithTokenResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appsid: ::windows_core::PCWSTR, msatarget: ::windows_core::PCWSTR, msapolicy: ::windows_core::PCWSTR, httpmethod: ::windows_core::PCWSTR, uri: ::windows_core::PCWSTR, headers: ::windows_core::PCWSTR, body: *const u8, bodysize: u32, forcerefresh: super::Foundation::BOOL, result: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetUserlessTokenAndSignatureWithTokenResult: usize,
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[repr(transparent)]
pub struct IXblIdpAuthTokenResult(::windows_core::IUnknown);
impl IXblIdpAuthTokenResult {
    pub unsafe fn GetStatus(&self) -> ::windows_core::Result<XBL_IDP_AUTH_TOKEN_STATUS> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStatus)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetErrorCode(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetErrorCode)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetToken(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetToken)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSignature(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSignature)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSandbox(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSandbox)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetEnvironment(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetEnvironment)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMsaAccountId(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetMsaAccountId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetXuid(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetXuid)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetGamertag(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetGamertag)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAgeGroup(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAgeGroup)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPrivileges(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPrivileges)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMsaTarget(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetMsaTarget)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMsaPolicy(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetMsaPolicy)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMsaAppId(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetMsaAppId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRedirect(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRedirect)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMessage(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetMessage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetHelpId(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetHelpId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetEnforcementBans(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetEnforcementBans)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRestrictions(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRestrictions)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTitleRestrictions(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTitleRestrictions)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXblIdpAuthTokenResult, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IXblIdpAuthTokenResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXblIdpAuthTokenResult {}
impl ::core::fmt::Debug for IXblIdpAuthTokenResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXblIdpAuthTokenResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IXblIdpAuthTokenResult {
    type Vtable = IXblIdpAuthTokenResult_Vtbl;
}
impl ::core::clone::Clone for IXblIdpAuthTokenResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IXblIdpAuthTokenResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x46ce0225_f267_4d68_b299_b2762552dec1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXblIdpAuthTokenResult_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: *mut XBL_IDP_AUTH_TOKEN_STATUS) -> ::windows_core::HRESULT,
    pub GetErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errorcode: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub GetToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetSignature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signature: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetSandbox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sandbox: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetEnvironment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, environment: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetMsaAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, msaaccountid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetXuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xuid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetGamertag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gamertag: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetAgeGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, agegroup: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetPrivileges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, privileges: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetMsaTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, msatarget: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetMsaPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, msapolicy: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetMsaAppId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, msaappid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetRedirect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, redirect: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetHelpId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, helpid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetEnforcementBans: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enforcementbans: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetRestrictions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, restrictions: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetTitleRestrictions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, titlerestrictions: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[repr(transparent)]
pub struct IXblIdpAuthTokenResult2(::windows_core::IUnknown);
impl IXblIdpAuthTokenResult2 {
    pub unsafe fn GetModernGamertag(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetModernGamertag)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetModernGamertagSuffix(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetModernGamertagSuffix)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetUniqueModernGamertag(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetUniqueModernGamertag)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXblIdpAuthTokenResult2, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IXblIdpAuthTokenResult2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXblIdpAuthTokenResult2 {}
impl ::core::fmt::Debug for IXblIdpAuthTokenResult2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXblIdpAuthTokenResult2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IXblIdpAuthTokenResult2 {
    type Vtable = IXblIdpAuthTokenResult2_Vtbl;
}
impl ::core::clone::Clone for IXblIdpAuthTokenResult2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IXblIdpAuthTokenResult2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x75d760b0_60b9_412d_994f_26b2cd5f7812);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXblIdpAuthTokenResult2_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetModernGamertag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetModernGamertagSuffix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetUniqueModernGamertag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GAMESTATS_OPEN_CREATED: GAMESTATS_OPEN_RESULT = GAMESTATS_OPEN_RESULT(0i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GAMESTATS_OPEN_OPENED: GAMESTATS_OPEN_RESULT = GAMESTATS_OPEN_RESULT(1i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GAMESTATS_OPEN_OPENONLY: GAMESTATS_OPEN_TYPE = GAMESTATS_OPEN_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GAMESTATS_OPEN_OPENORCREATE: GAMESTATS_OPEN_TYPE = GAMESTATS_OPEN_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GAMING_DEVICE_DEVICE_ID_NONE: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(0i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(1988865574i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE_S: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(712204761i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE_X: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(1523980231i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE_X_DEVKIT: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(284675555i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GAMING_DEVICE_DEVICE_ID_XBOX_SERIES_S: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(489159355i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GAMING_DEVICE_DEVICE_ID_XBOX_SERIES_X: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(796540415i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GAMING_DEVICE_DEVICE_ID_XBOX_SERIES_X_DEVKIT: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(-561359263i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GAMING_DEVICE_VENDOR_ID_MICROSOFT: GAMING_DEVICE_VENDOR_ID = GAMING_DEVICE_VENDOR_ID(-1024700366i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GAMING_DEVICE_VENDOR_ID_NONE: GAMING_DEVICE_VENDOR_ID = GAMING_DEVICE_VENDOR_ID(0i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GIS_ALL_USERS: GAME_INSTALL_SCOPE = GAME_INSTALL_SCOPE(3i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GIS_CURRENT_USER: GAME_INSTALL_SCOPE = GAME_INSTALL_SCOPE(2i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GIS_NOT_INSTALLED: GAME_INSTALL_SCOPE = GAME_INSTALL_SCOPE(1i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GameExplorer: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9a5ea990_3034_4d6f_9128_01f3c61022bc);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GameStatistics: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdbc85a2c_c0dc_4961_b6e2_d28b62c11ad4);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const ID_GDF_THUMBNAIL_STR: ::windows_core::PCWSTR = ::windows_core::w!("__GDF_THUMBNAIL");
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const ID_GDF_XML_STR: ::windows_core::PCWSTR = ::windows_core::w!("__GDF_XML");
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XBL_IDP_AUTH_TOKEN_STATUS_LOAD_MSA_ACCOUNT_FAILED: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(3i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XBL_IDP_AUTH_TOKEN_STATUS_MSA_INTERRUPT: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(5i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XBL_IDP_AUTH_TOKEN_STATUS_NO_ACCOUNT_SET: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(2i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XBL_IDP_AUTH_TOKEN_STATUS_OFFLINE_NO_CONSENT: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(6i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XBL_IDP_AUTH_TOKEN_STATUS_OFFLINE_SUCCESS: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(1i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XBL_IDP_AUTH_TOKEN_STATUS_SUCCESS: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(0i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XBL_IDP_AUTH_TOKEN_STATUS_UNKNOWN: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(-1i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XBL_IDP_AUTH_TOKEN_STATUS_VIEW_NOT_SET: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(7i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XBL_IDP_AUTH_TOKEN_STATUS_XBOX_VETO: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(4i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_ADD_FRIEND: KnownGamingPrivileges = KnownGamingPrivileges(255i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_BROADCAST: KnownGamingPrivileges = KnownGamingPrivileges(190i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_CLOUD_GAMING_JOIN_SESSION: KnownGamingPrivileges = KnownGamingPrivileges(208i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_CLOUD_GAMING_MANAGE_SESSION: KnownGamingPrivileges = KnownGamingPrivileges(207i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_CLOUD_SAVED_GAMES: KnownGamingPrivileges = KnownGamingPrivileges(209i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_COMMUNICATIONS: KnownGamingPrivileges = KnownGamingPrivileges(252i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_COMMUNICATION_VOICE_INGAME: KnownGamingPrivileges = KnownGamingPrivileges(205i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_COMMUNICATION_VOICE_SKYPE: KnownGamingPrivileges = KnownGamingPrivileges(206i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_GAME_DVR: KnownGamingPrivileges = KnownGamingPrivileges(198i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_MULTIPLAYER_PARTIES: KnownGamingPrivileges = KnownGamingPrivileges(203i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_MULTIPLAYER_SESSIONS: KnownGamingPrivileges = KnownGamingPrivileges(254i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_PREMIUM_CONTENT: KnownGamingPrivileges = KnownGamingPrivileges(214i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_PREMIUM_VIDEO: KnownGamingPrivileges = KnownGamingPrivileges(224i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_PROFILE_VIEWING: KnownGamingPrivileges = KnownGamingPrivileges(249i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_PURCHASE_CONTENT: KnownGamingPrivileges = KnownGamingPrivileges(245i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_SHARE_CONTENT: KnownGamingPrivileges = KnownGamingPrivileges(211i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_SHARE_KINECT_CONTENT: KnownGamingPrivileges = KnownGamingPrivileges(199i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_SOCIAL_NETWORK_SHARING: KnownGamingPrivileges = KnownGamingPrivileges(220i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_SUBSCRIPTION_CONTENT: KnownGamingPrivileges = KnownGamingPrivileges(219i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_USER_CREATED_CONTENT: KnownGamingPrivileges = KnownGamingPrivileges(247i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_VIDEO_COMMUNICATIONS: KnownGamingPrivileges = KnownGamingPrivileges(235i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_VIEW_FRIENDS_LIST: KnownGamingPrivileges = KnownGamingPrivileges(197i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XblIdpAuthManager: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xce23534b_56d8_4978_86a2_7ee570640468);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XblIdpAuthTokenResult: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9f493441_744a_410c_ae2b_9a22f7c7731f);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GAMESTATS_OPEN_RESULT(pub i32);
impl ::core::marker::Copy for GAMESTATS_OPEN_RESULT {}
impl ::core::clone::Clone for GAMESTATS_OPEN_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GAMESTATS_OPEN_RESULT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GAMESTATS_OPEN_RESULT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GAMESTATS_OPEN_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GAMESTATS_OPEN_RESULT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GAMESTATS_OPEN_TYPE(pub i32);
impl ::core::marker::Copy for GAMESTATS_OPEN_TYPE {}
impl ::core::clone::Clone for GAMESTATS_OPEN_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GAMESTATS_OPEN_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GAMESTATS_OPEN_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GAMESTATS_OPEN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GAMESTATS_OPEN_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GAME_INSTALL_SCOPE(pub i32);
impl ::core::marker::Copy for GAME_INSTALL_SCOPE {}
impl ::core::clone::Clone for GAME_INSTALL_SCOPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GAME_INSTALL_SCOPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GAME_INSTALL_SCOPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GAME_INSTALL_SCOPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GAME_INSTALL_SCOPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GAMING_DEVICE_DEVICE_ID(pub i32);
impl ::core::marker::Copy for GAMING_DEVICE_DEVICE_ID {}
impl ::core::clone::Clone for GAMING_DEVICE_DEVICE_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GAMING_DEVICE_DEVICE_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GAMING_DEVICE_DEVICE_ID {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GAMING_DEVICE_DEVICE_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GAMING_DEVICE_DEVICE_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GAMING_DEVICE_VENDOR_ID(pub i32);
impl ::core::marker::Copy for GAMING_DEVICE_VENDOR_ID {}
impl ::core::clone::Clone for GAMING_DEVICE_VENDOR_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GAMING_DEVICE_VENDOR_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GAMING_DEVICE_VENDOR_ID {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GAMING_DEVICE_VENDOR_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GAMING_DEVICE_VENDOR_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KnownGamingPrivileges(pub i32);
impl ::core::marker::Copy for KnownGamingPrivileges {}
impl ::core::clone::Clone for KnownGamingPrivileges {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KnownGamingPrivileges {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for KnownGamingPrivileges {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for KnownGamingPrivileges {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KnownGamingPrivileges").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XBL_IDP_AUTH_TOKEN_STATUS(pub i32);
impl ::core::marker::Copy for XBL_IDP_AUTH_TOKEN_STATUS {}
impl ::core::clone::Clone for XBL_IDP_AUTH_TOKEN_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XBL_IDP_AUTH_TOKEN_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for XBL_IDP_AUTH_TOKEN_STATUS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for XBL_IDP_AUTH_TOKEN_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XBL_IDP_AUTH_TOKEN_STATUS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
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
impl ::core::fmt::Debug for GAMING_DEVICE_MODEL_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GAMING_DEVICE_MODEL_INFORMATION").field("vendorId", &self.vendorId).field("deviceId", &self.deviceId).finish()
    }
}
impl ::windows_core::TypeKind for GAMING_DEVICE_MODEL_INFORMATION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for GAMING_DEVICE_MODEL_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.vendorId == other.vendorId && self.deviceId == other.deviceId
    }
}
impl ::core::cmp::Eq for GAMING_DEVICE_MODEL_INFORMATION {}
impl ::core::default::Default for GAMING_DEVICE_MODEL_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub type GameUICompletionRoutine = ::core::option::Option<unsafe extern "system" fn(returncode: ::windows_core::HRESULT, context: *const ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub type PlayerPickerUICompletionRoutine = ::core::option::Option<unsafe extern "system" fn(returncode: ::windows_core::HRESULT, context: *const ::core::ffi::c_void, selectedxuids: *const ::windows_core::HSTRING, selectedxuidscount: usize) -> ()>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
