/*++ BUILD Version: 0001    Increment this if a change has global effects

Copyright (c) 1985-1999, Microsoft Corporation

Module Name:

    winwlx.h

Abstract:

    WLX == WinLogon eXtension

    This file contains definitions, data types, and routine prototypes
    necessary to produce a replacement Graphical Identification aNd
    Authentication (GINA) DLL for Winlogon.

Author:

    Richard Ward (RichardW) and Jim Kelly (JimK) May-1994

Revision History:



--*/

#ifndef _WINWLX_
#define _WINWLX_

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)




////////////////////////////////////////////////////////////////////////
//                                                                    //
//  #defines                                                          //
//                                                                    //
////////////////////////////////////////////////////////////////////////


/////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////
//
// Revisions of Winlogon API available for use by GINAs
// Version is two parts: Major revision and minor revision.
// Major revision is the upper 16-bits, minor is the lower
// 16-bits.
//

#define WLX_VERSION_1_0             (0X00010000)
#define WLX_VERSION_1_1             (0X00010001)
#define WLX_VERSION_1_2             (0X00010002)
#define WLX_VERSION_1_3             (0X00010003)
#define WLX_VERSION_1_4             (0X00010004)
#define WLX_CURRENT_VERSION         (WLX_VERSION_1_4)


/////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////
//
// Secure attention sequence types
// These values are passed to routines that have a dwSasType
// parameter.
//
//  ALL VALUES FROM 0 TO 127 ARE RESERVED FOR MICROSOFT DEFINITION.
//  VALUES ABOVE 127 ARE RESERVED FOR CUSTOMER DEFINITION.
//
//      CTRL_ALT_DEL - used to indicate that the standard ctrl-alt-del
//          secure attention sequence has been entered.
//
//      SCRNSVR_TIMEOUT - used to indicate that keyboard/mouse inactivity
//          has lead to a screensaver activation.  It is up to the GINA
//          DLL whether this constitutes a workstation locking event.
//
//      SCRNSVR_ACTIVITY - used to indicate that keyboard or mouse
//          activity occured while a secure screensaver was active.
//
//      SC_INSERT - used to indicate that a smart card has been inserted
//          to a compatible device
//
//      SC_REMOVE - used to indicate that a smart card has been removed
//          from a compatible device
//

#define WLX_SAS_TYPE_TIMEOUT                    (0)
#define WLX_SAS_TYPE_CTRL_ALT_DEL               (1)
#define WLX_SAS_TYPE_SCRNSVR_TIMEOUT            (2)
#define WLX_SAS_TYPE_SCRNSVR_ACTIVITY           (3)
#define WLX_SAS_TYPE_USER_LOGOFF                (4)
#define WLX_SAS_TYPE_SC_INSERT                  (5)
#define WLX_SAS_TYPE_SC_REMOVE                  (6)
#define WLX_SAS_TYPE_AUTHENTICATED              (7)
#define WLX_SAS_TYPE_SC_FIRST_READER_ARRIVED    (8)
#define WLX_SAS_TYPE_SC_LAST_READER_REMOVED     (9)
#define WLX_SAS_TYPE_SWITCHUSER                 (10)
#define WLX_SAS_TYPE_MAX_MSFT_VALUE             (127)


//
// This structure is available through WlxGetOption, and is
// passed as the lParam for any S/C SAS notices sent to windows
//
typedef struct _WLX_SC_NOTIFICATION_INFO {
    PWSTR   pszCard ;
    PWSTR   pszReader ;
    PWSTR   pszContainer ;
    PWSTR   pszCryptoProvider ;
} WLX_SC_NOTIFICATION_INFO, * PWLX_SC_NOTIFICATION_INFO ;



/////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////
//
// Upon successful logon, the GINA DLL may specify any of the following
// options to Winlogon (via the dwOptions parameter of the WlxLoggedOutSas()
// api).  When set, these options specify:
//
//      NO_PROFILE - Winlogon must NOT load a profile for the logged
//                   on user.  Either the GINA DLL will take care of
//                   this activity, or the user does not need a profile.
//

#define WLX_LOGON_OPT_NO_PROFILE        (0x00000001)



/////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////
//
// GINA DLLs are expected to return account information to Winlogon
// following a successful logon.  This information allows Winlogon
// to support profile loading and supplemental network providers.
//
// To allow different sets of profile information to be returned
// by GINAs over time, the first DWORD of each profile structure
// is expected to contain a type-identifier.  The following constants
// are the defined profile type identifiers.
//

//
// Standard profile is V2_0
//

#define WLX_PROFILE_TYPE_V1_0           (1)
#define WLX_PROFILE_TYPE_V2_0           (2)



/////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////
//
// WlxLoggedOnSas() and WlxWkstaLockedSas() return an action
// value to Winlogon directing Winlogon to either remain unchanged
// or to perform some action (such as force-log the user off).
// These are the values that may be returned.  Note, however, that
// not all of the values may be returned by both of these api.  See
// the description of each api to see which values are expected from
// each.
//
//  LOGON              - User has logged on
//  NONE               - Don't change the state of the window station.
//  LOCK_WKSTA         - Lock the workstation, wait for next SAS.
//  LOGOFF             - Log the user off of the workstation.
//  SHUTDOWN           - Log the user off and shutdown the machine.
//  PWD_CHANGED        - Indicates that the user changed their password.  Notify network providers.
//  TASKLIST           - Invoke the task list.
//  UNLOCK_WKSTA       - Unlock the workstation.
//  FORCE_LOGOFF       - Forcibly log the user off.
//  SHUTDOWN_POWER_OFF - Turn off machine after shutting down.
//  SHUTDOWN_REBOOT    - Reboot machine after shutting down.
//  SHUTDOWN_SLEEP     - Put the machine to sleep
//  SHUTDOWN_SLEEP2    - Put the machine to sleep and disable wakeup events
//  SHUTDOWN_HIBERNATE - Hibernate the machine
//  RECONNECTED        - Session was reconnected to an earlier session
//

#define WLX_SAS_ACTION_LOGON                        (1)
#define WLX_SAS_ACTION_NONE                         (2)
#define WLX_SAS_ACTION_LOCK_WKSTA                   (3)
#define WLX_SAS_ACTION_LOGOFF                       (4)
#define WLX_SAS_ACTION_SHUTDOWN                     (5)
#define WLX_SAS_ACTION_PWD_CHANGED                  (6)
#define WLX_SAS_ACTION_TASKLIST                     (7)
#define WLX_SAS_ACTION_UNLOCK_WKSTA                 (8)
#define WLX_SAS_ACTION_FORCE_LOGOFF                 (9)
#define WLX_SAS_ACTION_SHUTDOWN_POWER_OFF           (10)
#define WLX_SAS_ACTION_SHUTDOWN_REBOOT              (11)
#define WLX_SAS_ACTION_SHUTDOWN_SLEEP               (12)
#define WLX_SAS_ACTION_SHUTDOWN_SLEEP2              (13)
#define WLX_SAS_ACTION_SHUTDOWN_HIBERNATE           (14)
#define WLX_SAS_ACTION_RECONNECTED                  (15)
#define WLX_SAS_ACTION_DELAYED_FORCE_LOGOFF         (16)
#define WLX_SAS_ACTION_SWITCH_CONSOLE               (17)


////////////////////////////////////////////////////////////////////////
//                                                                    //
//  Window Messages                                                   //
//                                                                    //
////////////////////////////////////////////////////////////////////////

//
// The WM_SAS is defined as follows
//
//  The wParam parameter has the SAS Type (above)

#define WLX_WM_SAS                  (WM_USER + 601)


//
// Dialog return values
//
// These may be returned by dialogs started by a GINA dll.
//
#define WLX_DLG_SAS                     101
#define WLX_DLG_INPUT_TIMEOUT           102     // Input (keys, etc) timed out
#define WLX_DLG_SCREEN_SAVER_TIMEOUT    103     // Screen Saver activated
#define WLX_DLG_USER_LOGOFF             104     // User logged off




////////////////////////////////////////////////////////////////////////
//                                                                    //
//  #data types                                                       //
//                                                                    //
////////////////////////////////////////////////////////////////////////


/////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////
//
// The WLX_PROFILE_* structure is returned from a GINA DLL
// following authentication.  This information is used by Winlogon
// to support supplemental Network Providers and to load the
// newly logged-on user's profile.
//
// Winlogon is responsible for freeing both the profile structure
// and the fields within the structure that are marked as separately
// deallocatable.
//

typedef struct _WLX_PROFILE_V1_0 {

    //
    // This field identifies the type of profile being returned by a
    // GINA DLL.  Profile types are defined with the prefix
    // WLX_PROFILE_TYPE_xxx.  It allows Winlogon to typecast the
    // structure so the remainder of the structure may be referenced.
    //

    DWORD               dwType;



    //
    // pathname of profile to load for user.
    //
    // The buffer pointed to by this field must be separately allocated.
    // Winlogon will free the buffer when it is no longer needed.
    //
    //
    PWSTR               pszProfile;

} WLX_PROFILE_V1_0, * PWLX_PROFILE_V1_0;


typedef struct _WLX_PROFILE_V2_0 {

    //
    // This field identifies the type of profile being returned by a
    // GINA DLL.  Profile types are defined with the prefix
    // WLX_PROFILE_TYPE_xxx.  It allows Winlogon to typecast the
    // structure so the remainder of the structure may be referenced.
    //

    DWORD               dwType;


    //
    // pathname of profile to load for user.
    //
    // This parameter can be NULL.  If so, the user has a local
    // profile only.
    //
    // The buffer pointed to by this field must be separately allocated.
    // Winlogon will free the buffer when it is no longer needed.
    //
    //

    PWSTR               pszProfile;


    //
    // pathname of policy to load for user.
    //
    // This parameter can be NULL which prevents network wide policy
    // from being applied.
    //
    // The buffer pointed to by this field must be separately allocated.
    // Winlogon will free the buffer when it is no longer needed.
    //
    //

    PWSTR               pszPolicy;


    //
    // pathname of network default user profile
    //
    // This parameter can be NULL, which causes the Default User
    // profile on the local machine to be used.
    //
    // The buffer pointed to by this field must be separately allocated.
    // Winlogon will free the buffer when it is no longer needed.
    //
    //

    PWSTR               pszNetworkDefaultUserProfile;


    //
    // name of the server which validated the user account
    //
    // This is used to enumerate globals groups the user belongs
    // to for policy support.  This parameter can be NULL.
    //
    // The buffer pointed to by this field must be separately allocated.
    // Winlogon will free the buffer when it is no longer needed.
    //
    //

    PWSTR               pszServerName;


    //
    // pointer to a series of null terminated environment variables
    //
    // envname=environment variable value
    //   - or -
    // envname=%OtherVar%\more text
    //
    // Each environment variable is NULL terminated with the last
    // environment variable double NULL terminated.  These variables
    // are set into the user's initial environment.  The environment
    // variable value can contain other environment variables wrapped
    // in "%" signs. This parameter can be NULL.
    //
    // The buffer pointed to by this field must be separately allocated.
    // Winlogon will free the buffer when it is no longer needed.
    //
    //

    PWSTR               pszEnvironment;

} WLX_PROFILE_V2_0, * PWLX_PROFILE_V2_0;



/////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////
//
// The WLX_NPR_NOTIFICATION_INFO structure is returned
// from a GINA DLL following successful authentication.
// This information is used by Winlogon to provide
// identification and authentication information already
// collected to network providers.  Winlogon is
// responsible for freeing both the main structure and all
// string and other buffers pointed to from within the
// structure.
//

typedef struct _WLX_MPR_NOTIFY_INFO {

    //
    // The name of the account logged onto (e.g. REDMOND\Joe).
    // The string pointed to by this field must be separately
    // allocated and will be separately deallocated by Winlogon.
    //

    PWSTR           pszUserName;

    //
    // The string pointed to by this field must be separately
    // allocated and will be separately deallocated by Winlogon.
    //

    PWSTR           pszDomain;

    //
    // Cleartext password of the user account.  If the OldPassword
    // field is non-null, then this field contains the new password
    // in a password change operation.  The string pointed to by
    // this field must be separately allocated and will be seperately
    // deallocated by Winlogon.
    //

    PWSTR           pszPassword;

    //
    // Cleartext old password of the user account whose password
    // has just been changed.  The Password field contains the new
    // password.  The string pointed to by this field must be
    // separately allocated and will be separately deallocated by
    // Winlogon.
    //

    PWSTR           pszOldPassword;

} WLX_MPR_NOTIFY_INFO, * PWLX_MPR_NOTIFY_INFO;



/////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////
//
// WLX_TERMINAL_SERVICES_DATA is used by the GINA during a
// WlxQueryTerminalServicesData() callback into WinLogon from the
// WlxLoggedOutSAS() context, after the user name and domain are known.
// This structure relates to TS user configuration information which is
// retrieved from the Domain Controller and SAM database. Having WinLogon
// pass this information means the GINA does not need to do the same
// off-machines lookups again.
//

#define WLX_DIRECTORY_LENGTH 256

typedef struct _WLX_TERMINAL_SERVICES_DATA {

    //
    // TS profile path, overrides the standard profile path.
    //

    WCHAR           ProfilePath[WLX_DIRECTORY_LENGTH + 1];


    //
    // TS home directory, overrides standard home directory.
    //

    WCHAR           HomeDir[WLX_DIRECTORY_LENGTH + 1];


    //
    // TS home directory drive, overrides standard drive.
    //

    WCHAR           HomeDirDrive[4];

} WLX_TERMINAL_SERVICES_DATA, *PWLX_TERMINAL_SERVICES_DATA;



/////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////
//
// The WLX_CLIENT_CREDENTIALS_INFO structure is returned
// from winlogon from the WlxQueryClientCredentials() call.
//
// This allows a network client WinStation to pass client
// credentials for automatic logon.
//
// The MSGINA DLL is responsible for freeing the memory
// and substrings with LocalFree().
//

#define WLX_CREDENTIAL_TYPE_V1_0            (1)
#define WLX_CREDENTIAL_TYPE_V2_0            (2)

typedef struct _WLX_CLIENT_CREDENTIALS_INFO {

    //
    // This field identifies the type of credentials structure being allocated
    // by GINA DLL.  Credential types are defined with the prefix
    // WLX_CREDENTIAL_TYPE_xxx.  It allows Winlogon to typecast the
    // structure so the remainder of the structure may be referenced.
    //

    DWORD           dwType;

    PWSTR           pszUserName;
    PWSTR           pszDomain;
    PWSTR           pszPassword;

    //
    // This field forces a prompt for the password. This
    // is due to an administrator override.
    //
    // This allows the distinguishing of autologon
    // with no password.
    //
    BOOL            fPromptForPassword;

} WLX_CLIENT_CREDENTIALS_INFO_V1_0, * PWLX_CLIENT_CREDENTIALS_INFO_V1_0;

typedef struct _WLX_CLIENT_CREDENTIALS_INFO_2_0 {

    DWORD           dwType;
    PWSTR           pszUserName;
    PWSTR           pszDomain;
    PWSTR           pszPassword;
    BOOL            fPromptForPassword;

    //
    // This field tells winlogon to disconnect/abort the logon attempt if the
    // provided password is incorrect, or if it should reprompt (current
    // behavior)
    //

    BOOL            fDisconnectOnLogonFailure;

} WLX_CLIENT_CREDENTIALS_INFO_V2_0, * PWLX_CLIENT_CREDENTIALS_INFO_V2_0;


/////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////
//
// The WLX_CONSOLESWITCH_CREDENTIALS_INFO structure is returned
// from gina in response to WlxGetConsoleSwitchCredentials calls.

// This structure is also returned from winlogon in response to 
// to WlxQueryConsoleSwitchCredentials call
//
// This is used to implement single session Terminal Server. A remote 
// session winlogon calls WlxGetConsoleSwitchCredentials to get the token
// and other info of the logged on user from msgina. This info is then passed to
// the console session winlogon to autologon the user on the console session.
// The gina on console session calls WlxQueryConsoleSwitchCredentials to get 
// this info from winlogon and logs on the user.
//
// The caller is responsible for freeing the memory
// and substrings with LocalFree().
//


#define WLX_CONSOLESWITCHCREDENTIAL_TYPE_V1_0            (1)

typedef struct _WLX_CONSOLESWITCH_CREDENTIALS_INFO {

    //
    // This field identifies the type of credentials structure being allocated
    // Credential types are defined with the prefix
    // WLX_CONSOLESWITCHCREDENTIAL_TYPE_xxx.  It allows Winlogon to typecast the
    // structure so the remainder of the structure may be referenced.
    //

    DWORD            dwType;

    HANDLE           UserToken;
    LUID             LogonId;
    QUOTA_LIMITS     Quotas;
    PWSTR            UserName;
    PWSTR            Domain;
    LARGE_INTEGER    LogonTime;
    BOOL             SmartCardLogon;
    ULONG            ProfileLength;

    //
    // From MSV1_0_INTERACTIVE_PROFILE 
    //
    DWORD            MessageType;
    USHORT           LogonCount;
    USHORT           BadPasswordCount;
    LARGE_INTEGER    ProfileLogonTime;
    LARGE_INTEGER    LogoffTime;
    LARGE_INTEGER    KickOffTime;
    LARGE_INTEGER    PasswordLastSet;
    LARGE_INTEGER    PasswordCanChange;
    LARGE_INTEGER    PasswordMustChange;
    PWSTR            LogonScript;
    PWSTR            HomeDirectory;
    PWSTR            FullName;
    PWSTR            ProfilePath;
    PWSTR            HomeDirectoryDrive;
    PWSTR            LogonServer;
    ULONG            UserFlags;
    ULONG            PrivateDataLen;
    PBYTE            PrivateData;

} WLX_CONSOLESWITCH_CREDENTIALS_INFO_V1_0, * PWLX_CONSOLESWITCH_CREDENTIALS_INFO_V1_0;


////////////////////////////////////////////////////////////////////////
//                                                                    //
//  Services that replacement GINAs   ** MUST ** provide              //
//                                                                    //
////////////////////////////////////////////////////////////////////////

BOOL
WINAPI
WlxNegotiate(
    DWORD                   dwWinlogonVersion,
    PDWORD                  pdwDllVersion
    );

BOOL
WINAPI
WlxInitialize(
    LPWSTR                  lpWinsta,
    HANDLE                  hWlx,
    PVOID                   pvReserved,
    PVOID                   pWinlogonFunctions,
    PVOID *                 pWlxContext
    );

VOID
WINAPI
WlxDisplaySASNotice(
    PVOID                   pWlxContext
    );


int
WINAPI
WlxLoggedOutSAS(
    PVOID                   pWlxContext,
    DWORD                   dwSasType,
    PLUID                   pAuthenticationId,
    PSID                    pLogonSid,
    PDWORD                  pdwOptions,
    PHANDLE                 phToken,
    PWLX_MPR_NOTIFY_INFO    pNprNotifyInfo,
    PVOID *                 pProfile
    );

BOOL
WINAPI
WlxActivateUserShell(
    PVOID                   pWlxContext,
    PWSTR                   pszDesktopName,
    PWSTR                   pszMprLogonScript,
    PVOID                   pEnvironment
    );

int
WINAPI
WlxLoggedOnSAS(
    PVOID                   pWlxContext,
    DWORD                   dwSasType,
    PVOID                   pReserved
    );

VOID
WINAPI
WlxDisplayLockedNotice(
    PVOID                   pWlxContext
    );

int
WINAPI
WlxWkstaLockedSAS(
    PVOID                   pWlxContext,
    DWORD                   dwSasType
    );

BOOL
WINAPI
WlxIsLockOk(
    PVOID                   pWlxContext
    );

BOOL
WINAPI
WlxIsLogoffOk(
    PVOID                   pWlxContext
    );

VOID
WINAPI
WlxLogoff(
    PVOID                   pWlxContext
    );


VOID
WINAPI
WlxShutdown(
    PVOID                   pWlxContext,
    DWORD                   ShutdownType
    );


//
// NEW for version 1.1
//
BOOL
WINAPI
WlxScreenSaverNotify(
    PVOID                   pWlxContext,
    BOOL *                  pSecure);

BOOL
WINAPI
WlxStartApplication(
    PVOID                   pWlxContext,
    PWSTR                   pszDesktopName,
    PVOID                   pEnvironment,
    PWSTR                   pszCmdLine
    );

//
// New for 1.3
//

BOOL
WINAPI
WlxNetworkProviderLoad(
    PVOID                   pWlxContext,
    PWLX_MPR_NOTIFY_INFO    pNprNotifyInfo
    );


#define STATUSMSG_OPTION_NOANIMATION    0x00000001
#define STATUSMSG_OPTION_SETFOREGROUND  0x00000002

BOOL
WINAPI
WlxDisplayStatusMessage(
    PVOID                   pWlxContext,
    HDESK                   hDesktop,
    DWORD                   dwOptions,
    PWSTR                   pTitle,
    PWSTR                   pMessage
    );

BOOL
WINAPI
WlxGetStatusMessage(
    PVOID                   pWlxContext,
    DWORD *                 pdwOptions,
    PWSTR                   pMessage,
    DWORD                   dwBufferSize
    );

BOOL
WINAPI
WlxRemoveStatusMessage(
    PVOID                   pWlxContext
    );


//
// New for 1.4
//
BOOL
WINAPI
WlxGetConsoleSwitchCredentials (
    PVOID                   pWlxContext,
    PVOID                   pCredInfo
    );

VOID
WINAPI
WlxReconnectNotify (
    PVOID                   pWlxContext
    );

VOID
WINAPI
WlxDisconnectNotify (
    PVOID                   pWlxContext
    );


////////////////////////////////////////////////////////////////////////
//                                                                    //
//  Services that Winlogon provides                                   //
//                                                                    //
////////////////////////////////////////////////////////////////////////

typedef struct _WLX_DESKTOP {
    DWORD       Size;
    DWORD       Flags;
    HDESK       hDesktop;
    PWSTR       pszDesktopName;
} WLX_DESKTOP, * PWLX_DESKTOP;

#define WLX_DESKTOP_NAME    0x00000001      // Name present
#define WLX_DESKTOP_HANDLE  0x00000002      // Handle present



typedef VOID
(WINAPI * PWLX_USE_CTRL_ALT_DEL)(
    HANDLE                  hWlx
    );

typedef VOID
(WINAPI * PWLX_SET_CONTEXT_POINTER)(
    HANDLE                  hWlx,
    PVOID                   pWlxContext
    );

typedef VOID
(WINAPI * PWLX_SAS_NOTIFY)(
    HANDLE                  hWlx,
    DWORD                   dwSasType
    );

typedef BOOL
(WINAPI * PWLX_SET_TIMEOUT)(
    HANDLE                  hWlx,
    DWORD                   Timeout);

typedef int
(WINAPI * PWLX_ASSIGN_SHELL_PROTECTION)(
    HANDLE                  hWlx,
    HANDLE                  hToken,
    HANDLE                  hProcess,
    HANDLE                  hThread
    );

typedef int
(WINAPI * PWLX_MESSAGE_BOX)(
    HANDLE                  hWlx,
    HWND                    hwndOwner,
    LPWSTR                  lpszText,
    LPWSTR                  lpszTitle,
    UINT                    fuStyle
    );

typedef int
(WINAPI * PWLX_DIALOG_BOX)(
    HANDLE                  hWlx,
    HANDLE                  hInst,
    LPWSTR                  lpszTemplate,
    HWND                    hwndOwner,
    DLGPROC                 dlgprc
    );

typedef int
(WINAPI * PWLX_DIALOG_BOX_INDIRECT)(
    HANDLE                  hWlx,
    HANDLE                  hInst,
    LPCDLGTEMPLATE          hDialogTemplate,
    HWND                    hwndOwner,
    DLGPROC                 dlgprc
    );

typedef int
(WINAPI * PWLX_DIALOG_BOX_PARAM)(
    HANDLE                  hWlx,
    HANDLE                  hInst,
    LPWSTR                  lpszTemplate,
    HWND                    hwndOwner,
    DLGPROC                 dlgprc,
    LPARAM                  dwInitParam
    );

typedef int
(WINAPI * PWLX_DIALOG_BOX_INDIRECT_PARAM)(
    HANDLE                  hWlx,
    HANDLE                  hInst,
    LPCDLGTEMPLATE          hDialogTemplate,
    HWND                    hwndOwner,
    DLGPROC                 dlgprc,
    LPARAM                  dwInitParam
    );

typedef int
(WINAPI * PWLX_SWITCH_DESKTOP_TO_USER)(
    HANDLE                  hWlx);

typedef int
(WINAPI * PWLX_SWITCH_DESKTOP_TO_WINLOGON)(
    HANDLE                  hWlx);


typedef int
(WINAPI * PWLX_CHANGE_PASSWORD_NOTIFY)(
    HANDLE                  hWlx,
    PWLX_MPR_NOTIFY_INFO    pMprInfo,
    DWORD                   dwChangeInfo
    );

typedef BOOL
(WINAPI * PWLX_GET_SOURCE_DESKTOP)(
    HANDLE                  hWlx,
    PWLX_DESKTOP *          ppDesktop);

typedef BOOL
(WINAPI * PWLX_SET_RETURN_DESKTOP)(
    HANDLE                  hWlx,
    PWLX_DESKTOP            pDesktop);

typedef BOOL
(WINAPI * PWLX_CREATE_USER_DESKTOP)(
    HANDLE                  hWlx,
    HANDLE                  hToken,
    DWORD                   Flags,
    PWSTR                   pszDesktopName,
    PWLX_DESKTOP *          ppDesktop);

#define WLX_CREATE_INSTANCE_ONLY    0x00000001
#define WLX_CREATE_USER             0x00000002

typedef int
(WINAPI * PWLX_CHANGE_PASSWORD_NOTIFY_EX)(
    HANDLE                  hWlx,
    PWLX_MPR_NOTIFY_INFO    pMprInfo,
    DWORD                   dwChangeInfo,
    PWSTR                   ProviderName,
    PVOID                   Reserved);

typedef BOOL
(WINAPI * PWLX_CLOSE_USER_DESKTOP)(
    HANDLE          hWlx,
    PWLX_DESKTOP    pDesktop,
    HANDLE          hToken );

typedef BOOL
(WINAPI * PWLX_SET_OPTION)(
    HANDLE hWlx,
    DWORD Option,
    ULONG_PTR Value,
    ULONG_PTR * OldValue
    );

typedef BOOL
(WINAPI * PWLX_GET_OPTION)(
    HANDLE hWlx,
    DWORD Option,
    ULONG_PTR * Value
    );


typedef VOID
(WINAPI * PWLX_WIN31_MIGRATE)(
    HANDLE                  hWlx
    );

typedef BOOL
(WINAPI * PWLX_QUERY_CLIENT_CREDENTIALS)(
    PWLX_CLIENT_CREDENTIALS_INFO_V1_0 pCred
    );

typedef BOOL
(WINAPI * PWLX_QUERY_IC_CREDENTIALS)(
    PWLX_CLIENT_CREDENTIALS_INFO_V1_0 pCred
    );

typedef BOOL
(WINAPI * PWLX_QUERY_TS_LOGON_CREDENTIALS)(
    PWLX_CLIENT_CREDENTIALS_INFO_V2_0 pCred
    );

typedef BOOL
(WINAPI * PWLX_DISCONNECT)(
    );

typedef DWORD
(WINAPI * PWLX_QUERY_TERMINAL_SERVICES_DATA)(
    HANDLE hWlx,
    PWLX_TERMINAL_SERVICES_DATA pTSData,
    WCHAR * UserName,
    WCHAR * Domain
    );

typedef DWORD
(WINAPI * PWLX_QUERY_CONSOLESWITCH_CREDENTIALS)(
      PWLX_CONSOLESWITCH_CREDENTIALS_INFO_V1_0 pCred
     );


//
// Options that can be get or set:
//

#define WLX_OPTION_USE_CTRL_ALT_DEL     0x00000001
#define WLX_OPTION_CONTEXT_POINTER      0x00000002
#define WLX_OPTION_USE_SMART_CARD       0x00000003
#define WLX_OPTION_FORCE_LOGOFF_TIME    0x00000004
#define WLX_OPTION_IGNORE_AUTO_LOGON    0x00000008
#define WLX_OPTION_NO_SWITCH_ON_SAS     0x00000009

//
// Options that can be queried only:
//

#define WLX_OPTION_SMART_CARD_PRESENT   0x00010001
#define WLX_OPTION_SMART_CARD_INFO      0x00010002
#define WLX_OPTION_DISPATCH_TABLE_SIZE  0x00010003



////////////////////////////////////////////////////////////////////////
//                                                                    //
//  Function dispatch tables.                                         //
//  One of the following tables will be passed to the GINA DLL        //
//  in the WlxInitialize() call during initialization.                //
//                                                                    //
////////////////////////////////////////////////////////////////////////

typedef struct _WLX_DISPATCH_VERSION_1_0 {
    PWLX_USE_CTRL_ALT_DEL           WlxUseCtrlAltDel;
    PWLX_SET_CONTEXT_POINTER        WlxSetContextPointer;
    PWLX_SAS_NOTIFY                 WlxSasNotify;
    PWLX_SET_TIMEOUT                WlxSetTimeout;
    PWLX_ASSIGN_SHELL_PROTECTION    WlxAssignShellProtection;
    PWLX_MESSAGE_BOX                WlxMessageBox;
    PWLX_DIALOG_BOX                 WlxDialogBox;
    PWLX_DIALOG_BOX_PARAM           WlxDialogBoxParam;
    PWLX_DIALOG_BOX_INDIRECT        WlxDialogBoxIndirect;
    PWLX_DIALOG_BOX_INDIRECT_PARAM  WlxDialogBoxIndirectParam;
    PWLX_SWITCH_DESKTOP_TO_USER     WlxSwitchDesktopToUser;
    PWLX_SWITCH_DESKTOP_TO_WINLOGON WlxSwitchDesktopToWinlogon;
    PWLX_CHANGE_PASSWORD_NOTIFY     WlxChangePasswordNotify;
} WLX_DISPATCH_VERSION_1_0, *PWLX_DISPATCH_VERSION_1_0;

typedef struct _WLX_DISPATCH_VERSION_1_1 {
    PWLX_USE_CTRL_ALT_DEL           WlxUseCtrlAltDel;
    PWLX_SET_CONTEXT_POINTER        WlxSetContextPointer;
    PWLX_SAS_NOTIFY                 WlxSasNotify;
    PWLX_SET_TIMEOUT                WlxSetTimeout;
    PWLX_ASSIGN_SHELL_PROTECTION    WlxAssignShellProtection;
    PWLX_MESSAGE_BOX                WlxMessageBox;
    PWLX_DIALOG_BOX                 WlxDialogBox;
    PWLX_DIALOG_BOX_PARAM           WlxDialogBoxParam;
    PWLX_DIALOG_BOX_INDIRECT        WlxDialogBoxIndirect;
    PWLX_DIALOG_BOX_INDIRECT_PARAM  WlxDialogBoxIndirectParam;
    PWLX_SWITCH_DESKTOP_TO_USER     WlxSwitchDesktopToUser;
    PWLX_SWITCH_DESKTOP_TO_WINLOGON WlxSwitchDesktopToWinlogon;
    PWLX_CHANGE_PASSWORD_NOTIFY     WlxChangePasswordNotify;
    PWLX_GET_SOURCE_DESKTOP         WlxGetSourceDesktop;
    PWLX_SET_RETURN_DESKTOP         WlxSetReturnDesktop;
    PWLX_CREATE_USER_DESKTOP        WlxCreateUserDesktop;
    PWLX_CHANGE_PASSWORD_NOTIFY_EX  WlxChangePasswordNotifyEx;
} WLX_DISPATCH_VERSION_1_1, * PWLX_DISPATCH_VERSION_1_1;

typedef struct _WLX_DISPATCH_VERSION_1_2 {
    PWLX_USE_CTRL_ALT_DEL           WlxUseCtrlAltDel;
    PWLX_SET_CONTEXT_POINTER        WlxSetContextPointer;
    PWLX_SAS_NOTIFY                 WlxSasNotify;
    PWLX_SET_TIMEOUT                WlxSetTimeout;
    PWLX_ASSIGN_SHELL_PROTECTION    WlxAssignShellProtection;
    PWLX_MESSAGE_BOX                WlxMessageBox;
    PWLX_DIALOG_BOX                 WlxDialogBox;
    PWLX_DIALOG_BOX_PARAM           WlxDialogBoxParam;
    PWLX_DIALOG_BOX_INDIRECT        WlxDialogBoxIndirect;
    PWLX_DIALOG_BOX_INDIRECT_PARAM  WlxDialogBoxIndirectParam;
    PWLX_SWITCH_DESKTOP_TO_USER     WlxSwitchDesktopToUser;
    PWLX_SWITCH_DESKTOP_TO_WINLOGON WlxSwitchDesktopToWinlogon;
    PWLX_CHANGE_PASSWORD_NOTIFY     WlxChangePasswordNotify;
    PWLX_GET_SOURCE_DESKTOP         WlxGetSourceDesktop;
    PWLX_SET_RETURN_DESKTOP         WlxSetReturnDesktop;
    PWLX_CREATE_USER_DESKTOP        WlxCreateUserDesktop;
    PWLX_CHANGE_PASSWORD_NOTIFY_EX  WlxChangePasswordNotifyEx;
    PWLX_CLOSE_USER_DESKTOP         WlxCloseUserDesktop ;
} WLX_DISPATCH_VERSION_1_2, * PWLX_DISPATCH_VERSION_1_2;

typedef struct _WLX_DISPATCH_VERSION_1_3 {
    PWLX_USE_CTRL_ALT_DEL             WlxUseCtrlAltDel;
    PWLX_SET_CONTEXT_POINTER          WlxSetContextPointer;
    PWLX_SAS_NOTIFY                   WlxSasNotify;
    PWLX_SET_TIMEOUT                  WlxSetTimeout;
    PWLX_ASSIGN_SHELL_PROTECTION      WlxAssignShellProtection;
    PWLX_MESSAGE_BOX                  WlxMessageBox;
    PWLX_DIALOG_BOX                   WlxDialogBox;
    PWLX_DIALOG_BOX_PARAM             WlxDialogBoxParam;
    PWLX_DIALOG_BOX_INDIRECT          WlxDialogBoxIndirect;
    PWLX_DIALOG_BOX_INDIRECT_PARAM    WlxDialogBoxIndirectParam;
    PWLX_SWITCH_DESKTOP_TO_USER       WlxSwitchDesktopToUser;
    PWLX_SWITCH_DESKTOP_TO_WINLOGON   WlxSwitchDesktopToWinlogon;
    PWLX_CHANGE_PASSWORD_NOTIFY       WlxChangePasswordNotify;
    PWLX_GET_SOURCE_DESKTOP           WlxGetSourceDesktop;
    PWLX_SET_RETURN_DESKTOP           WlxSetReturnDesktop;
    PWLX_CREATE_USER_DESKTOP          WlxCreateUserDesktop;
    PWLX_CHANGE_PASSWORD_NOTIFY_EX    WlxChangePasswordNotifyEx;
    PWLX_CLOSE_USER_DESKTOP           WlxCloseUserDesktop ;
    PWLX_SET_OPTION                   WlxSetOption;
    PWLX_GET_OPTION                   WlxGetOption;
    PWLX_WIN31_MIGRATE                WlxWin31Migrate;
    PWLX_QUERY_CLIENT_CREDENTIALS     WlxQueryClientCredentials;
    PWLX_QUERY_IC_CREDENTIALS         WlxQueryInetConnectorCredentials;
    PWLX_DISCONNECT                   WlxDisconnect;
    PWLX_QUERY_TERMINAL_SERVICES_DATA WlxQueryTerminalServicesData;
} WLX_DISPATCH_VERSION_1_3, * PWLX_DISPATCH_VERSION_1_3;

typedef struct _WLX_DISPATCH_VERSION_1_4 {
    PWLX_USE_CTRL_ALT_DEL               WlxUseCtrlAltDel;
    PWLX_SET_CONTEXT_POINTER            WlxSetContextPointer;
    PWLX_SAS_NOTIFY                     WlxSasNotify;
    PWLX_SET_TIMEOUT                    WlxSetTimeout;
    PWLX_ASSIGN_SHELL_PROTECTION        WlxAssignShellProtection;
    PWLX_MESSAGE_BOX                    WlxMessageBox;
    PWLX_DIALOG_BOX                     WlxDialogBox;
    PWLX_DIALOG_BOX_PARAM               WlxDialogBoxParam;
    PWLX_DIALOG_BOX_INDIRECT            WlxDialogBoxIndirect;
    PWLX_DIALOG_BOX_INDIRECT_PARAM      WlxDialogBoxIndirectParam;
    PWLX_SWITCH_DESKTOP_TO_USER         WlxSwitchDesktopToUser;
    PWLX_SWITCH_DESKTOP_TO_WINLOGON     WlxSwitchDesktopToWinlogon;
    PWLX_CHANGE_PASSWORD_NOTIFY         WlxChangePasswordNotify;
    PWLX_GET_SOURCE_DESKTOP             WlxGetSourceDesktop;
    PWLX_SET_RETURN_DESKTOP             WlxSetReturnDesktop;
    PWLX_CREATE_USER_DESKTOP            WlxCreateUserDesktop;
    PWLX_CHANGE_PASSWORD_NOTIFY_EX      WlxChangePasswordNotifyEx;
    PWLX_CLOSE_USER_DESKTOP             WlxCloseUserDesktop ;
    PWLX_SET_OPTION                     WlxSetOption;
    PWLX_GET_OPTION                     WlxGetOption;
    PWLX_WIN31_MIGRATE                  WlxWin31Migrate;
    PWLX_QUERY_CLIENT_CREDENTIALS       WlxQueryClientCredentials;
    PWLX_QUERY_IC_CREDENTIALS           WlxQueryInetConnectorCredentials;
    PWLX_DISCONNECT                     WlxDisconnect;
    PWLX_QUERY_TERMINAL_SERVICES_DATA   WlxQueryTerminalServicesData;
    PWLX_QUERY_CONSOLESWITCH_CREDENTIALS WlxQueryConsoleSwitchCredentials;
    PWLX_QUERY_TS_LOGON_CREDENTIALS     WlxQueryTsLogonCredentials;
} WLX_DISPATCH_VERSION_1_4, * PWLX_DISPATCH_VERSION_1_4;


//
// Non-GINA notification DLLs
//

typedef DWORD (*PFNMSGECALLBACK)(BOOL bVerbose, LPWSTR lpMessage);

typedef _Struct_size_bytes_(Size) struct _WLX_NOTIFICATION_INFO {
    _Field_range_(>=, sizeof(struct _WLX_NOTIFICATION_INFO)) ULONG  Size ;
    ULONG  Flags ;
    PWSTR  UserName ;
    PWSTR  Domain ;
    PWSTR  WindowStation ;
    HANDLE hToken ;
    HDESK  hDesktop ;
    PFNMSGECALLBACK pStatusCallback ;
} WLX_NOTIFICATION_INFO, * PWLX_NOTIFICATION_INFO ;




#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif /* _WINWLX_ */
