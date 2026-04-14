//
// Copyright (c) Microsoft Corporation. All rights reserved.
//

//
// API Set Contract:
//
//    api-ms-win-gaming-tcui-l1-1-*
//
// Abstract:
//
//    This header file provides API function signatures for gaming related
//    TCUI APIs.
//

#ifdef MSC_VER
#pragma once
#endif

#ifndef _GAMING_TCUI_EXT_
#define _GAMING_TCUI_EXT_

#if NTDDI_VERSION >= NTDDI_THRESHOLD

#include <apiset.h>
#include <apisetcconv.h>
#ifdef _CONTRACT_GEN
#include <nt.h>
#include <ntrtl.h>
#include <nturtl.h>
#include <windows.h>
#include <Inspectable.h>
#endif

#include <winstring.h>

#if defined(__cplusplus)
extern "C" {
#endif

typedef VOID (WINAPI * GameUICompletionRoutine) (
    _In_ HRESULT returnCode,
    _In_ void* context
    );

typedef VOID (WINAPI * PlayerPickerUICompletionRoutine) (
    _In_ HRESULT returnCode,
    _In_ void* context,
    _In_reads_(selectedXuidsCount) const HSTRING* selectedXuids,
    _In_ size_t selectedXuidsCount
    );

HRESULT
WINAPI
ShowGameInviteUI(
    _In_ HSTRING serviceConfigurationId,
    _In_ HSTRING sessionTemplateName,
    _In_ HSTRING sessionId,
    _In_ HSTRING invitationDisplayText,
    _In_ GameUICompletionRoutine completionRoutine,
    _In_opt_ void* context
    );

HRESULT
WINAPI
ShowPlayerPickerUI(
    _In_ HSTRING promptDisplayText,
    _In_reads_(xuidsCount) const HSTRING* xuids,
    _In_ size_t xuidsCount,
    _In_reads_opt_(preSelectedXuidsCount) const HSTRING* preSelectedXuids,
    _In_ size_t preSelectedXuidsCount,
    _In_ size_t minSelectionCount,
    _In_ size_t maxSelectionCount,
    _In_ PlayerPickerUICompletionRoutine completionRoutine,
    _In_opt_ void* context
    );

HRESULT
WINAPI
ShowProfileCardUI(
    _In_ HSTRING targetUserXuid,
    _In_ GameUICompletionRoutine completionRoutine,
    _In_opt_ void* context
    );

HRESULT
WINAPI
ShowChangeFriendRelationshipUI(
    _In_ HSTRING targetUserXuid,
    _In_ GameUICompletionRoutine completionRoutine,
    _In_opt_ void* context
    );

HRESULT
WINAPI
ShowTitleAchievementsUI(
    _In_ UINT32 titleId,
    _In_ GameUICompletionRoutine completionRoutine,
    _In_opt_ void* context
    );

HRESULT
WINAPI
ProcessPendingGameUI(
    _In_ BOOL waitForCompletion
    );

BOOL
WINAPI
TryCancelPendingGameUI(
    );

typedef enum KnownGamingPrivileges{
    XPRIVILEGE_BROADCAST = 190,                     // The user can broadcast live gameplay
    XPRIVILEGE_VIEW_FRIENDS_LIST = 197,             // The user can view other user's friends list if this privilege is present
    XPRIVILEGE_GAME_DVR = 198,                      // The user can upload recorded in-game videos to the cloud if this privilege is present. Viewing GameDVRs is subject to privacy controls.
    XPRIVILEGE_SHARE_KINECT_CONTENT = 199,          // Kinect recorded content can be uploaded to the cloud for the user and made accessible to anyone if this privilege is present. Viewing other user's Kinect content is subject to a privacy setting.
    XPRIVILEGE_MULTIPLAYER_PARTIES = 203,           // The user can join a party session if this privilege is present
    XPRIVILEGE_COMMUNICATION_VOICE_INGAME = 205,    // The user can participate in voice chat during parties and multiplayer game sessions if this privilege is present. Communicating with other users is subject to additional privacy permission checks
    XPRIVILEGE_COMMUNICATION_VOICE_SKYPE = 206,     // The user can use voice communication with Skype on Xbox One if this privilege is present
    XPRIVILEGE_CLOUD_GAMING_MANAGE_SESSION = 207,   // The user can allocate a cloud compute cluster and manage a cloud compute cluster for a hosted game session if this privilege is present
    XPRIVILEGE_CLOUD_GAMING_JOIN_SESSION = 208,     // The user can join a cloud compute session if this privilege is present
    XPRIVILEGE_CLOUD_SAVED_GAMES = 209,             // The user can save games in cloud title storage if this privilege is present
    XPRIVILEGE_SHARE_CONTENT = 211,                 // The user can share content with others if this privilege is present
    XPRIVILEGE_PREMIUM_CONTENT = 214,               // The user can purchase, download and launch premium content available with the Xbox LIVE Gold subscription if this privilege is present
    XPRIVILEGE_SUBSCRIPTION_CONTENT = 219,          // The user can purchase and download premium subscription content and use premium subscription features when this privilege is present
    XPRIVILEGE_SOCIAL_NETWORK_SHARING = 220,        // The user is allowed to share progress information on social networks when this privilege is present
    XPRIVILEGE_PREMIUM_VIDEO = 224,                 // The user can access premium video services if this privilege is present
    XPRIVILEGE_VIDEO_COMMUNICATIONS = 235,          // The user can use video communication with Skype or other providers when this privilege is present. Communicating with other users is subject to additional privacy permission checks
    XPRIVILEGE_PURCHASE_CONTENT = 245,              // The user is authorized to purchase content when this privilege is present
    XPRIVILEGE_USER_CREATED_CONTENT = 247,          // The user is authorized to download and view online user created content when this privilege is present.
    XPRIVILEGE_PROFILE_VIEWING = 249,               // The user is authorized to view other user's profiles when this privilege is present. Viewing other user's profiles is subject to additional privacy checks
    XPRIVILEGE_COMMUNICATIONS = 252,                // The user can use asynchronous text messaging with anyone when this privilege is present. Extra privacy permissions checks are required to determine who the user is authorized to communicate with. Communicating with other users is subject to additional privacy permission checks
    XPRIVILEGE_MULTIPLAYER_SESSIONS = 254,          // The user can join a multiplayer sessions for a game when this privilege is present.
    XPRIVILEGE_ADD_FRIEND = 255,                    // The user can follow other Xbox LIVE users and add Xbox LIVE friends when this privilege is present
} KnownGamingPrivileges;

HRESULT
WINAPI
CheckGamingPrivilegeWithUI(
    _In_ UINT32 privilegeId,
    _In_ HSTRING scope,
    _In_ HSTRING policy,
    _In_opt_ HSTRING friendlyMessage,
    _In_ GameUICompletionRoutine completionRoutine,
    _In_opt_ void* context
    );

HRESULT
WINAPI
CheckGamingPrivilegeSilently(
    _In_ UINT32 privilegeId,
    _In_ HSTRING scope,
    _In_ HSTRING policy,
    _Out_ BOOL* hasPrivilege
    );

HRESULT
WINAPI
ShowGameInviteUIForUser(
    _In_ IInspectable* user,
    _In_ HSTRING serviceConfigurationId,
    _In_ HSTRING sessionTemplateName,
    _In_ HSTRING sessionId,
    _In_ HSTRING invitationDisplayText,
    _In_ GameUICompletionRoutine completionRoutine,
    _In_opt_ void* context
    );

HRESULT
WINAPI
ShowPlayerPickerUIForUser(
    _In_ IInspectable* user,
    _In_ HSTRING promptDisplayText,
    _In_reads_(xuidsCount) const HSTRING* xuids,
    _In_ size_t xuidsCount,
    _In_reads_opt_(preSelectedXuidsCount) const HSTRING* preSelectedXuids,
    _In_ size_t preSelectedXuidsCount,
    _In_ size_t minSelectionCount,
    _In_ size_t maxSelectionCount,
    _In_ PlayerPickerUICompletionRoutine completionRoutine,
    _In_opt_ void* context
    );

HRESULT
WINAPI
ShowProfileCardUIForUser(
    _In_ IInspectable* user,
    _In_ HSTRING targetUserXuid,
    _In_ GameUICompletionRoutine completionRoutine,
    _In_opt_ void* context
    );

HRESULT
WINAPI
ShowChangeFriendRelationshipUIForUser(
    _In_ IInspectable* user,
    _In_ HSTRING targetUserXuid,
    _In_ GameUICompletionRoutine completionRoutine,
    _In_opt_ void* context
    );

HRESULT
WINAPI
ShowTitleAchievementsUIForUser(
    _In_ IInspectable* user,
    _In_ UINT32 titleId,
    _In_ GameUICompletionRoutine completionRoutine,
    _In_opt_ void* context
    );

HRESULT
WINAPI
CheckGamingPrivilegeWithUIForUser(
    _In_ IInspectable* user,
    _In_ UINT32 privilegeId,
    _In_ HSTRING scope,
    _In_ HSTRING policy,
    _In_opt_ HSTRING friendlyMessage,
    _In_ GameUICompletionRoutine completionRoutine,
    _In_opt_ void* context
    );

HRESULT
WINAPI
CheckGamingPrivilegeSilentlyForUser(
    _In_ IInspectable* user,
    _In_ UINT32 privilegeId,
    _In_ HSTRING scope,
    _In_ HSTRING policy,
    _Out_ BOOL* hasPrivilege
    );

HRESULT
WINAPI
ShowGameInviteUIWithContext(
    _In_ HSTRING serviceConfigurationId,
    _In_ HSTRING sessionTemplateName,
    _In_ HSTRING sessionId,
    _In_ HSTRING invitationDisplayText,
    _In_ HSTRING customActivationContext,
    _In_ GameUICompletionRoutine completionRoutine,
    _In_opt_ void* context
    );

HRESULT
WINAPI
ShowGameInviteUIWithContextForUser(
    _In_ IInspectable* user,
    _In_ HSTRING serviceConfigurationId,
    _In_ HSTRING sessionTemplateName,
    _In_ HSTRING sessionId,
    _In_ HSTRING invitationDisplayText,
    _In_ HSTRING customActivationContext,
    _In_ GameUICompletionRoutine completionRoutine,
    _In_opt_ void* context
    );

HRESULT
WINAPI
ShowGameInfoUI(
    _In_ UINT32 titleId,
    _In_ GameUICompletionRoutine completionRoutine,
    _In_opt_ void* context
    );

HRESULT
WINAPI
ShowGameInfoUIForUser(
    _In_ IInspectable* user,
    _In_ UINT32 titleId,
    _In_ GameUICompletionRoutine completionRoutine,
    _In_opt_ void* context
    );

HRESULT
WINAPI
ShowFindFriendsUI(
    _In_ GameUICompletionRoutine completionRoutine,
    _In_opt_ void* context
    );

HRESULT
WINAPI
ShowFindFriendsUIForUser(
    _In_ IInspectable* user,
    _In_ GameUICompletionRoutine completionRoutine,
    _In_opt_ void* context
    );

HRESULT
WINAPI
ShowCustomizeUserProfileUI(
    _In_ GameUICompletionRoutine completionRoutine,
    _In_opt_ void* context
    );

HRESULT
WINAPI
ShowCustomizeUserProfileUIForUser(
    _In_ IInspectable* user,
    _In_ GameUICompletionRoutine completionRoutine,
    _In_opt_ void* context
    );

HRESULT
WINAPI
ShowUserSettingsUI(
    _In_ GameUICompletionRoutine completionRoutine,
    _In_opt_ void* context
    );

HRESULT
WINAPI
ShowUserSettingsUIForUser(
    _In_ IInspectable* user,
    _In_ GameUICompletionRoutine completionRoutine,
    _In_opt_ void* context
    );

#if defined(__cplusplus)
} // end extern "C"
#endif // defined(__cplusplus)

#endif // NTDDI_VERSION >= NTDDI_THRESHOLD

#endif // _GAMING_TCUI_EXT_
