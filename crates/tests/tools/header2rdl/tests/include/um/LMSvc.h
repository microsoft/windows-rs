/*++ BUILD Version: 0002    // Increment this if a change has global effects

Copyright (c) 1991-1999  Microsoft Corporation

Module Name:

    lmsvc.h

Abstract:

    This file contains structures, function prototypes, and definitions
    for the NetService API.

[Environment:]

    User Mode -Win32

[Notes:]

    You must include NETCONS.H before this file, since this file depends
    on values defined in NETCONS.H.

--*/

#ifndef _LMSVC_
#define _LMSVC_

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus
extern "C" {
#endif

//
// Include the file which contains all the service name strings.
//
#include <lmsname.h>

//
//  Data Structures
//

typedef struct _SERVICE_INFO_0 {
    LPWSTR  svci0_name;
} SERVICE_INFO_0, *PSERVICE_INFO_0, * LPSERVICE_INFO_0;

typedef struct _SERVICE_INFO_1 {
    LPWSTR  svci1_name;
    DWORD   svci1_status;
    DWORD   svci1_code;
    DWORD   svci1_pid;
} SERVICE_INFO_1, *PSERVICE_INFO_1, * LPSERVICE_INFO_1;

typedef struct _SERVICE_INFO_2 {
    LPWSTR  svci2_name;
    DWORD   svci2_status;
    DWORD   svci2_code;
    DWORD   svci2_pid;
    LPWSTR  svci2_text;
    DWORD   svci2_specific_error;
    LPWSTR  svci2_display_name;
} SERVICE_INFO_2, *PSERVICE_INFO_2, * LPSERVICE_INFO_2;

//
// Function Prototypes
//

NET_API_STATUS NET_API_FUNCTION
NetServiceControl (
    _In_opt_    LPCWSTR servername,
    _In_        LPCWSTR service,
    _In_        DWORD   opcode,
    _In_        DWORD   arg,
    _Outptr_ LPBYTE *bufptr
    );

NET_API_STATUS NET_API_FUNCTION
NetServiceEnum (
    _In_opt_    LPCWSTR  servername,
    _In_        DWORD    level,
    _Outptr_result_buffer_(_Inexpressible_(entriesread)) LPBYTE  *bufptr,
    _In_        DWORD    prefmaxlen,
    _Out_       LPDWORD  entriesread,
    _Out_       LPDWORD  totalentries,
    _Inout_opt_ LPDWORD  resume_handle
    );

NET_API_STATUS NET_API_FUNCTION
NetServiceGetInfo (
    _In_opt_    LPCWSTR  servername,
    _In_        LPCWSTR  service,
    _In_        DWORD    level,
    _Outptr_ LPBYTE  *bufptr
    );

NET_API_STATUS NET_API_FUNCTION
NetServiceInstall (
    _In_opt_          LPCWSTR  servername,
    _In_              LPCWSTR  service,
    _In_              DWORD    argc,
    _In_reads_(argc) LPCWSTR  argv[],
    _Outptr_       LPBYTE  *bufptr
    );

//
// Special Values and Constants
//

//
//  Bitmask and bit values for svci1_status, and svci2_status
//  fields.  For each "subfield", there is a mask defined,
//  and a number of constants representing the value
//  obtained by doing (status & mask).
//

// Bits 0,1 -- general status

#define SERVICE_INSTALL_STATE       0x03
#define SERVICE_UNINSTALLED         0x00
#define SERVICE_INSTALL_PENDING     0x01
#define SERVICE_UNINSTALL_PENDING   0x02
#define SERVICE_INSTALLED           0x03

// Bits 2,3 -- paused/active status

#define SERVICE_PAUSE_STATE              0x0C
#define LM20_SERVICE_ACTIVE              0x00
#define LM20_SERVICE_CONTINUE_PENDING    0x04
#define LM20_SERVICE_PAUSE_PENDING       0x08
#define LM20_SERVICE_PAUSED              0x0C

// Bit 4 -- uninstallable indication

#define SERVICE_NOT_UNINSTALLABLE   0x00
#define SERVICE_UNINSTALLABLE       0x10

// Bit 5 -- pausable indication

#define SERVICE_NOT_PAUSABLE        0x00
#define SERVICE_PAUSABLE            0x20

// Workstation service only:
// Bits 8,9,10 -- redirection paused/active

#define SERVICE_REDIR_PAUSED        0x700
#define SERVICE_REDIR_DISK_PAUSED   0x100
#define SERVICE_REDIR_PRINT_PAUSED  0x200
#define SERVICE_REDIR_COMM_PAUSED   0x400

//
//  Additional standard LAN Manager for MS-DOS services
//

#define SERVICE_DOS_ENCRYPTION  L"ENCRYPT"

//
//  NetServiceControl opcodes.
//

#define SERVICE_CTRL_INTERROGATE    0
#define SERVICE_CTRL_PAUSE          1
#define SERVICE_CTRL_CONTINUE       2
#define SERVICE_CTRL_UNINSTALL      3

//
//  Workstation service only:  Bits used in the "arg" parameter
//  to NetServiceControl in conjunction with the opcode
//  SERVICE_CTRL_PAUSE or SERVICE_CTRL_CONTINUE, to pause or
//  continue redirection.
//

#define SERVICE_CTRL_REDIR_DISK     0x1
#define SERVICE_CTRL_REDIR_PRINT    0x2
#define SERVICE_CTRL_REDIR_COMM     0x4

//
//  Values for svci1_code, and svci2_code when status
//  of the service is SERVICE_INSTALL_PENDING or
//  SERVICE_UNINSTALL_PENDING.
//  A service can optionally provide a hint to the installer
//  that the install is proceeding and how long to wait
//  (in 0.1 second increments) before querying status again.
//

#define SERVICE_IP_NO_HINT          0x0
#define SERVICE_CCP_NO_HINT         0x0

#define SERVICE_IP_QUERY_HINT       0x10000
#define SERVICE_CCP_QUERY_HINT      0x10000

//
// Mask for install proceeding checkpoint number
//

#define SERVICE_IP_CHKPT_NUM        0x0FF
#define SERVICE_CCP_CHKPT_NUM       0x0FF

//
// Mask for wait time hint before querying again
//

#define SERVICE_IP_WAIT_TIME        0x0FF00
#define SERVICE_CCP_WAIT_TIME       0x0FF00

//
// Shift count for building wait time _code values
//

#define SERVICE_IP_WAITTIME_SHIFT   8
#define SERVICE_NTIP_WAITTIME_SHIFT 12

//
// Mask used for upper and lower portions of wait hint time.
//
#define UPPER_HINT_MASK     0x0000FF00
#define LOWER_HINT_MASK     0x000000FF
#define UPPER_GET_HINT_MASK 0x0FF00000
#define LOWER_GET_HINT_MASK 0x0000FF00
#define SERVICE_NT_MAXTIME  0x0000FFFF
#define SERVICE_RESRV_MASK  0x0001FFFF
#define SERVICE_MAXTIME     0x000000FF

//
//  SERVICE_BASE is the base of service error codes,
//  chosen to avoid conflict with OS, redirector,
//  netapi, and errlog codes.
//
// Don't change the comments following the manifest constants without
// understanding how mapmsg works.
//

#define SERVICE_BASE                3050
#define SERVICE_UIC_NORMAL          0
/*
 *  Uninstall codes, to be used in high byte of 'code' on final NetStatus,
 *  which sets the status to UNINSTALLED.
 */

#define SERVICE_UIC_BADPARMVAL          (SERVICE_BASE + 1)
/*
 * The Registry or the information you just typed includes an illegal
 * value for "%1".
 */

#define SERVICE_UIC_MISSPARM            (SERVICE_BASE + 2)
/*
 * The required parameter was not provided on the command
 * line or in the configuration file.
 */

#define SERVICE_UIC_UNKPARM             (SERVICE_BASE + 3)
/*
 * LAN Manager does not recognize "%1" as a valid option.
 */

#define SERVICE_UIC_RESOURCE            (SERVICE_BASE + 4)
/*
 * A request for resource could not be satisfied.
 */

#define SERVICE_UIC_CONFIG              (SERVICE_BASE + 5)
/*
 * A problem exists with the system configuration.
 */

#define SERVICE_UIC_SYSTEM              (SERVICE_BASE + 6)
/*
 * A system error has occurred.
 */

#define SERVICE_UIC_INTERNAL            (SERVICE_BASE + 7)
/*
 * An internal consistency error has occurred.
 */

#define SERVICE_UIC_AMBIGPARM           (SERVICE_BASE + 8)
/*
 * The configuration file or the command line has an ambiguous option.
 */

#define SERVICE_UIC_DUPPARM             (SERVICE_BASE + 9)
/*
 * The configuration file or the command line has a duplicate parameter.
 */

#define SERVICE_UIC_KILL                (SERVICE_BASE + 10)
/*
 * The service did not respond to control and was stopped with
 * the DosKillProc function.
 */

#define SERVICE_UIC_EXEC                (SERVICE_BASE + 11)
/*
 * An error occurred when attempting to run the service program.
 */

#define SERVICE_UIC_SUBSERV             (SERVICE_BASE + 12)
/*
 * The sub-service failed to start.
 */

#define SERVICE_UIC_CONFLPARM           (SERVICE_BASE + 13)
/*
 * There is a conflict in the value or use of these options: %1.
 */

#define SERVICE_UIC_FILE                (SERVICE_BASE + 14)
/*
 * There is a problem with the file.
 */



//
//  The modifiers
//

//
// General:
//

#define SERVICE_UIC_M_NULL  0

//
//  RESOURCE:
//

#define SERVICE_UIC_M_MEMORY    (SERVICE_BASE + 20)     /* memory */
#define SERVICE_UIC_M_DISK      (SERVICE_BASE + 21)     /* disk space */
#define SERVICE_UIC_M_THREADS   (SERVICE_BASE + 22)     /* thread */
#define SERVICE_UIC_M_PROCESSES (SERVICE_BASE + 23)     /* process */

//
//  CONFIG:
//

//
// Security failure
//

#define SERVICE_UIC_M_SECURITY          (SERVICE_BASE + 24)
/* Security Failure. %0 */

#define SERVICE_UIC_M_LANROOT           (SERVICE_BASE + 25)
/*
 * Bad or missing LAN Manager root directory.
 */

#define SERVICE_UIC_M_REDIR             (SERVICE_BASE + 26)
/*
 * The network software is not installed.
 */

#define SERVICE_UIC_M_SERVER            (SERVICE_BASE + 27)
/*
 * The server is not started.
 */

#define SERVICE_UIC_M_SEC_FILE_ERR      (SERVICE_BASE + 28)
/*
 * The server cannot access the user accounts database (NET.ACC).
 */

#define SERVICE_UIC_M_FILES             (SERVICE_BASE + 29)
/*
 * Incompatible files are installed in the LANMAN tree.
 */

#define SERVICE_UIC_M_LOGS              (SERVICE_BASE + 30)
/*
 * The LANMAN\LOGS directory is invalid.
 */

#define SERVICE_UIC_M_LANGROUP          (SERVICE_BASE + 31)
/*
 * The domain specified could not be used.
 */

#define SERVICE_UIC_M_MSGNAME           (SERVICE_BASE + 32)
/*
 * The computer name is being used as a message alias on another computer.
 */

#define SERVICE_UIC_M_ANNOUNCE          (SERVICE_BASE + 33)
/*
 * The announcement of the server name failed.
 */

#define SERVICE_UIC_M_UAS               (SERVICE_BASE + 34)
/*
 * The user accounts database is not configured correctly.
 */

#define SERVICE_UIC_M_SERVER_SEC_ERR    (SERVICE_BASE + 35)
/*
 * The server is not running with user-level security.
 */

#define SERVICE_UIC_M_WKSTA             (SERVICE_BASE + 37)
/*
 * The workstation is not configured properly.
 */

#define SERVICE_UIC_M_ERRLOG            (SERVICE_BASE + 38)
/*
 * View your error log for details.
 */

#define SERVICE_UIC_M_FILE_UW           (SERVICE_BASE + 39)
/*
 * Unable to write to this file.
 */

#define SERVICE_UIC_M_ADDPAK            (SERVICE_BASE + 40)
/*
 * ADDPAK file is corrupted.  Delete LANMAN\NETPROG\ADDPAK.SER
 * and reapply all ADDPAKs.
 */

#define SERVICE_UIC_M_LAZY              (SERVICE_BASE + 41)
/*
 * The LM386 server cannot be started because CACHE.EXE is not running.
 */

#define SERVICE_UIC_M_UAS_MACHINE_ACCT  (SERVICE_BASE + 42)
/*
 * There is no account for this computer in the security database.
 */

#define SERVICE_UIC_M_UAS_SERVERS_NMEMB (SERVICE_BASE + 43)
/*
 * This computer is not a member of the group SERVERS.
 */

#define SERVICE_UIC_M_UAS_SERVERS_NOGRP (SERVICE_BASE + 44)
/*
 * The group SERVERS is not present in the local security database.
 */

#define SERVICE_UIC_M_UAS_INVALID_ROLE  (SERVICE_BASE + 45)
/*
 * This computer is configured as a member of a workgroup, not as
 * a member of a domain. The Netlogon service does not need to run in this
 * configuration.
 */

#define SERVICE_UIC_M_NETLOGON_NO_DC    (SERVICE_BASE + 46)
/*
 * The primary Domain Controller for this domain could not be located.
 */

#define SERVICE_UIC_M_NETLOGON_DC_CFLCT (SERVICE_BASE + 47)
/*
 * This computer is configured to be the primary domain controller of its domain.
 * However, the computer %1 is currently claiming to be the primary domain controller
 * of the domain.
 */

#define SERVICE_UIC_M_NETLOGON_AUTH     (SERVICE_BASE + 48)
/*
 * The service failed to authenticate with the primary domain controller.
 */

#define SERVICE_UIC_M_UAS_PROLOG        (SERVICE_BASE + 49)
/*
 * There is a problem with the security database creation date or serial number.
 */


#define SERVICE2_BASE    5600
/* new SEVICE_UIC messages go here */

#define SERVICE_UIC_M_NETLOGON_MPATH    (SERVICE2_BASE + 0)
/*
 * Could not share the User or Script path.
 */

#define SERVICE_UIC_M_LSA_MACHINE_ACCT  (SERVICE2_BASE + 1)
/*
 * The password for this computer is not found in the local security
 * database.
 */

#define SERVICE_UIC_M_DATABASE_ERROR    (SERVICE2_BASE + 2)
/*
 * An internal error occurred while accessing the computer's
 * local or network security database.
 */


//
//  End modifiers
//

//
// Commonly used Macros:
//

#define SERVICE_IP_CODE(tt,nn) \
  ((long)SERVICE_IP_QUERY_HINT|(long)(nn|(tt<<SERVICE_IP_WAITTIME_SHIFT)))

#define SERVICE_CCP_CODE(tt,nn) \
  ((long)SERVICE_CCP_QUERY_HINT|(long)(nn|(tt<<SERVICE_IP_WAITTIME_SHIFT)))

#define SERVICE_UIC_CODE(cc,mm) \
  ((long)(((long)cc<<16)|(long)(unsigned short)mm))

//
// This macro takes a wait hint (tt) which can have a maximum value of
// 0xFFFF and puts it into the service status code field.
// 0x0FF1FFnn  (where nn is the checkpoint information).
//
#define SERVICE_NT_CCP_CODE(tt,nn)  \
  (  \
    ((long)SERVICE_CCP_QUERY_HINT)   | \
    ((long)(nn))   | \
    (((tt)&LOWER_HINT_MASK) << SERVICE_IP_WAITTIME_SHIFT)   | \
    (((tt)&UPPER_HINT_MASK) << SERVICE_NTIP_WAITTIME_SHIFT)   \
  )

//
// This macro takes a status code field, and strips out the wait hint
// from the upper and lower sections.
// 0x0FF1FFnn results in 0x0000FFFF.
//
#define SERVICE_NT_WAIT_GET(code) \
    (   \
      (((code) & UPPER_GET_HINT_MASK) >> SERVICE_NTIP_WAITTIME_SHIFT)  |  \
      (((code) & LOWER_GET_HINT_MASK) >> SERVICE_IP_WAITTIME_SHIFT)  \
    )

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _LMSVC_
