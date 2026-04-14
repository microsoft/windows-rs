/*++ BUILD Version: 0003    // Increment this if a change has global effects

Copyright (c) 1990-1999  Microsoft Corporation


    LMCONS.H (was NETCONS.H in LM 2.x)

Abstract:

    This file contains constants used throughout the LAN Manager
    API header files.  It should be included in any source file
    that is going to include other LAN Manager API header files or
    call a LAN Manager API.

    NOTE:  Lengths of strings are given as the maximum lengths of the
    string in characters (not bytes).  This does not include space for the
    terminating 0-characters.  When allocating space for such an item,
    use the form:

        TCHAR username[UNLEN+1];

    Definitions of the form LN20_* define those values in effect for
    LanMan 2.0.



--*/

/*NOINC*/
#ifndef NETCONS_INCLUDED

#define NETCONS_INCLUDED
#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)


/*INC*/

#ifndef PASCAL
#define PASCAL                          // pascal on OS/2
#endif

#ifndef FAR
#define FAR                             // far on OS/2
#endif

//
// String Lengths for various LanMan names
//

#define CNLEN       15                  // Computer name length
#define LM20_CNLEN  15                  // LM 2.0 Computer name length
#define DNLEN       CNLEN               // Maximum domain name length
#define LM20_DNLEN  LM20_CNLEN          // LM 2.0 Maximum domain name length

#if (CNLEN != DNLEN)
#error CNLEN and DNLEN are not equal
#endif

#define UNCLEN      (CNLEN+2)           // UNC computer name length
#define LM20_UNCLEN (LM20_CNLEN+2)      // LM 2.0 UNC computer name length

#define NNLEN       80                  // Net name length (share name)
#define LM20_NNLEN  12                  // LM 2.0 Net name length

#define RMLEN       (UNCLEN+1+NNLEN)    // Max remote name length
#define LM20_RMLEN  (LM20_UNCLEN+1+LM20_NNLEN) // LM 2.0 Max remote name length

#define SNLEN       80                  // Service name length
#define LM20_SNLEN  15                  // LM 2.0 Service name length
#define STXTLEN     256                 // Service text length
#define LM20_STXTLEN 63                 // LM 2.0 Service text length

#define PATHLEN     256                 // Max. path (not including drive name)
#define LM20_PATHLEN 256                // LM 2.0 Max. path

#define DEVLEN      80                  // Device name length
#define LM20_DEVLEN 8                   // LM 2.0 Device name length

#define EVLEN       16                  // Event name length

//
// User, Group and Password lengths
//

#define UNLEN       256                 // Maximum user name length
#define LM20_UNLEN  20                  // LM 2.0 Maximum user name length

#define GNLEN       UNLEN               // Group name
#define LM20_GNLEN  LM20_UNLEN          // LM 2.0 Group name

#define PWLEN       256                 // Maximum password length
#define LM20_PWLEN  14                  // LM 2.0 Maximum password length

#define SHPWLEN     8                   // Share password length (bytes)


#define CLTYPE_LEN  12                  // Length of client type string


#define MAXCOMMENTSZ 256                // Multipurpose comment length
#define LM20_MAXCOMMENTSZ 48            // LM 2.0 Multipurpose comment length

#define QNLEN       NNLEN               // Queue name maximum length
#define LM20_QNLEN  LM20_NNLEN          // LM 2.0 Queue name maximum length
#if (QNLEN != NNLEN)
# error QNLEN and NNLEN are not equal
#endif

//
// The ALERTSZ and MAXDEVENTRIES defines have not yet been NT'ized.
// Whoever ports these components should change these values appropriately.
//

#define ALERTSZ     128                 // size of alert string in server
#define MAXDEVENTRIES (sizeof (int)*8)  // Max number of device entries

                                        //
                                        // We use int bitmap to represent
                                        //

#define NETBIOS_NAME_LEN  16            // NetBIOS net name (bytes)

//
// Value to be used with APIs which have a "preferred maximum length"
// parameter.  This value indicates that the API should just allocate
// "as much as it takes."
//

#define MAX_PREFERRED_LENGTH    ((DWORD) -1)

//
// Moral equivalent of DNS_MAX_NAME_LENGTH in windnsdef.h.
//
#define LM_DNS_MAX_NAME_LENGTH  255

//
//        Constants used with encryption
//

#define CRYPT_KEY_LEN           7
#define CRYPT_TXT_LEN           8
#define ENCRYPTED_PWLEN         16
#define SESSION_PWLEN           24
#define SESSION_CRYPT_KLEN      21

//
//  Value to be used with SetInfo calls to allow setting of all
//  settable parameters (parmnum zero option)
//
#ifndef PARMNUM_ALL
#define PARMNUM_ALL             0
#endif

#define PARM_ERROR_UNKNOWN      ( (DWORD) (-1) )
#define PARM_ERROR_NONE         0
#define PARMNUM_BASE_INFOLEVEL  1000

//
// Only the UNICODE version of the LM APIs are available on NT.
// Non-UNICODE version on other platforms
//
#if defined( _WIN32_WINNT ) || defined( WINNT ) || defined( __midl ) \
    || defined( FORCE_UNICODE )
#define LMSTR   LPWSTR
#define LMCSTR  LPCWSTR
#else
#define LMSTR   LPSTR
#define LMCSTR  LPCSTR
#endif

//
//        Message File Names
//

#define MESSAGE_FILENAME        TEXT("NETMSG")
#define OS2MSG_FILENAME         TEXT("BASE")
#define HELP_MSG_FILENAME       TEXT("NETH")

/**INTERNAL_ONLY**/

// The backup message file named here is a duplicate of net.msg. It
// is not shipped with the product, but is used at buildtime to
// msgbind certain messages to netapi.dll and some of the services.
// This allows for OEMs to modify the message text in net.msg and
// have those changes show up.        Only in case there is an error in
// retrieving the messages from net.msg do we then get the bound
// messages out of bak.msg (really out of the message segment).

#define BACKUP_MSG_FILENAME     TEXT("BAK.MSG")

/**END_INTERNAL**/

#ifndef NULL
#ifdef __cplusplus
#define NULL	0
#else
#define NULL	((void *)0)
#endif
#endif

// Define pseudo-keywords.
#ifndef IN
#define IN
#endif

#ifndef OPTIONAL
#define OPTIONAL
#endif

#ifndef OUT
#define OUT
#endif
/*INC*/



//
// The platform ID indicates the levels to use for platform-specific
// information.
//

#define PLATFORM_ID_DOS 300
#define PLATFORM_ID_OS2 400
#define PLATFORM_ID_NT  500
#define PLATFORM_ID_OSF 600
#define PLATFORM_ID_VMS 700

//
//      There message numbers assigned to different LANMAN components
//      are as defined below.
//
//      lmerr.h:        2100 - 2999     NERR_BASE
//      alertmsg.h:     3000 - 3049     ALERT_BASE
//      lmsvc.h:        3050 - 3099     SERVICE_BASE
//      lmerrlog.h:     3100 - 3299     ERRLOG_BASE
//      msgtext.h:      3300 - 3499     MTXT_BASE
//      apperr.h:       3500 - 3999     APPERR_BASE
//      apperrfs.h:     4000 - 4299     APPERRFS_BASE
//      apperr2.h:      4300 - 5299     APPERR2_BASE
//      ncberr.h:       5300 - 5499     NRCERR_BASE
//      alertmsg.h:     5500 - 5599     ALERT2_BASE
//      lmsvc.h:        5600 - 5699     SERVICE2_BASE
//      lmerrlog.h      5700 - 5899     ERRLOG2_BASE
//

#define MIN_LANMAN_MESSAGE_ID  NERR_BASE
#define MAX_LANMAN_MESSAGE_ID  5899

/*NOINC*/

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family or App Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)

//
// Keywords used in Function Prototypes
//

#define NET_API_STATUS          DWORD
#define API_RET_TYPE            NET_API_STATUS      // Old value: do not use
#if (_MSC_VER >= 800) || defined(_STDCALL_SUPPORTED)
#define NET_API_FUNCTION    __stdcall
#else
#define NET_API_FUNCTION
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#endif // NETCONS_INCLUDED
/*INC*/
